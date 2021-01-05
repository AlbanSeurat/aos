/*
 * MIT License
 *
 * Copyright (c) 2018-2019 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::gpio;
use crate::{delays, mbox, IRQ, BCMDeviceMemory};
use core::{
    ops,
    sync::atomic::{compiler_fence, Ordering},
};
use cortex_a::asm;
use register::{mmio::*, register_bitfields};
use crate::io::{Writer, Reader, IoResult};

// PL011 UART registers.
//
// Descriptions taken from
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    /// Flag Register
    FR [
        /// Transmit FIFO full. The meaning of this bit depends on the
        /// state of the FEN bit in the UARTLCR_ LCRH Register. If the
        /// FIFO is disabled, this bit is set when the transmit
        /// holding register is full. If the FIFO is enabled, the TXFF
        /// bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the
        /// state of the FEN bit in the UARTLCR_H Register. If the
        /// FIFO is disabled, this bit is set when the receive holding
        /// register is empty. If the FIFO is enabled, the RXFE bit is
        /// set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) []
    ],

    /// Integer Baud rate divisor
    IBRD [
        /// Integer Baud rate divisor
        IBRD OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud rate divisor
    FBRD [
        /// Fractional Baud rate divisor
        FBRD OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control register
    LCRH [
        /// Word length. These bits indicate the number of data bits
        /// transmitted or received in a frame.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        /// Enable FIFOs:
        ///
        /// 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding
        /// registers
        ///
        /// 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    /// Control Register
    CR [
        /// Receive enable. If this bit is set to 1, the receive
        /// section of the UART is enabled. Data reception occurs for
        /// UART signals. When the UART is disabled in the middle of
        /// reception, it completes the current character before
        /// stopping.
        RXE    OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit
        /// section of the UART is enabled. Data transmission occurs
        /// for UART signals. When the UART is disabled in the middle
        /// of transmission, it completes the current character before
        /// stopping.
        TXE    OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission
            /// or reception, it completes the current character
            /// before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interupt Clear Register
    ICR [
        /// Meta field for all pending interrupts
        ALL OFFSET(0) NUMBITS(11) []
    ],

    /// Interrupt mask set/clear register
    IMSC [
        /// Meta field for all pending interrupts
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    DR: ReadWrite<u32>,                   // 0x00
    __reserved_0: [u32; 5],               // 0x04
    FR: ReadOnly<u32, FR::Register>,      // 0x18
    __reserved_1: [u32; 2],               // 0x1c
    IBRD: WriteOnly<u32, IBRD::Register>, // 0x24
    FBRD: WriteOnly<u32, FBRD::Register>, // 0x28
    LCRH: WriteOnly<u32, LCRH::Register>, // 0x2C
    CR: WriteOnly<u32, CR::Register>,     // 0x30
    IFLS: ReadWrite<u32>,                 // 0x34
    IMSC: ReadWrite<u32, IMSC::Register>, // 0x38
    __reserved_2: [u32; 2],               // 0x3c
    ICR: WriteOnly<u32, ICR::Register>,   // 0x44
}

pub enum UartError {
    MailboxError,
}
pub type ResultUart<T> = ::core::result::Result<T, UartError>;

#[derive(Copy, Clone)]
pub struct Uart {
    base_addr: usize,
}

impl ops::Deref for Uart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl Uart {
    pub const fn new(base_addr: usize) -> Uart {
        Uart { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    ///Set baud rate and characteristics (115200 8N1) and map to GPIO
    pub fn init(
        &self,
        v_mbox: &mut mbox::Mbox,
        gpio: &gpio::GPIO,
    ) -> ResultUart<()> {
        // turn off UART0
        self.CR.set(0);

        // set up clock for consistent divisor values
        v_mbox.buffer[0] = 9 * 4;
        v_mbox.buffer[1] = mbox::REQUEST;
        v_mbox.buffer[2] = mbox::tag::SETCLKRATE;
        v_mbox.buffer[3] = 12;
        v_mbox.buffer[4] = 8;
        v_mbox.buffer[5] = mbox::clock::UART; // UART clock
        v_mbox.buffer[6] = 4_000_000; // 4Mhz
        v_mbox.buffer[7] = 0; // skip turbo setting
        v_mbox.buffer[8] = mbox::tag::LAST;

        // Insert a compiler fence that ensures that all stores to the
        // mbox buffer are finished before the GPU is signaled (which
        // is done by a store operation as well).
        compiler_fence(Ordering::Release);

        if v_mbox.call(mbox::channel::PROP).is_err() {
            return Err(UartError::MailboxError); // Abort if UART clocks couldn't be set
        };

        // map UART0 to GPIO pins
        gpio.GPFSEL1
            .modify(gpio::GPFSEL1::FSEL14::TXD0 + gpio::GPFSEL1::FSEL15::RXD0);

        gpio.GPPUD.set(0); // enable pins 14 and 15
        delays::wait_cycles(150);

        gpio.GPPUDCLK0.modify(
            gpio::GPPUDCLK0::PUDCLK14::AssertClock + gpio::GPPUDCLK0::PUDCLK15::AssertClock,
        );
        delays::wait_cycles(150);

        gpio.GPPUDCLK0.set(0);

        self.ICR.write(ICR::ALL::CLEAR);
        self.IBRD.write(IBRD::IBRD.val(2)); // Results in 115200 baud
        self.FBRD.write(FBRD::FBRD.val(0xB));
        self.LCRH.write(LCRH::WLEN::EightBit); // 8N1
        self.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Ok(())
    }

    pub unsafe fn enable_rx_irq(&self, irq : &IRQ, bcm: &BCMDeviceMemory) {
        self.IMSC.set(1 << 4);
        irq.external_enable(1 << 25);
    }

    fn putc(&self, c: u8) {
        // wait until we can send
        loop {
            if !self.FR.is_set(FR::TXFF) {
                break;
            }
            asm::nop();
        }

        // write the character to the buffer
        self.DR.set(c as u32);
    }

}

impl Writer for Uart {

    /// Display a string
    fn write(&mut self, bytes: &[u8]) -> IoResult<usize> {
        let mut inc = 0usize;
        for c in bytes {
            self.putc(*c);
            inc = inc + 1;
        }
        Ok(inc)
    }
}

impl Reader for Uart {
    fn clear(&mut self) -> IoResult<u8> {
        let mut pos = 0;
        while !self.FR.matches_all(FR::RXFE::SET) {
            self.DR.get();
            pos = pos + 1;
        }
        Ok(pos)
    }

    fn read_char(&mut self) -> IoResult<u8> {
        while self.FR.matches_all(FR::RXFE::SET) {
            asm::nop();
        }
        // Read one character.
        Ok(self.DR.get() as u8)
    }

    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let mut pos = 0;
        while pos < buf.len() {
            buf[pos] = self.read_char()?;
            pos = pos + 1;
        }
        Ok(pos)
    }
}

