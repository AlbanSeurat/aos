/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
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

use core::{ops, mem};
use tock_registers::{
    registers::{ReadOnly, WriteOnly},
    register_bitfields,
};
use crate::{DMA, mbox};
use crate::dma::SliceAllocator;
use core::arch::asm;
use tock_registers::interfaces::{Readable, Writeable};

register_bitfields! {
    u32,

    STATUS [
        FULL  OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    READ: ReadOnly<u32>,
    // 0x00
    __reserved_0: [u32; 5],
    // 0x04
    STATUS: ReadOnly<u32, STATUS::Register>,
    // 0x18
    __reserved_1: u32,
    // 0x1C
    WRITE: WriteOnly<u32>,                   // 0x20
}

// Custom errors
#[derive(Debug)]
pub enum MboxError {
    ResponseError,
    UnknownError,
}

// Channels
pub mod channel {
    pub const PROP: u32 = 8;
}

// Tags
#[allow(dead_code)]
pub mod tag {
    pub const GETSERIAL: u32 = 0x10004;
    pub const SETCLKRATE: u32 = 0x38002;

    pub const GET_SCREEN_FRAME_BUFFER: u32 = 0x40001;
    pub const GET_PITCH: u32 = 0x40008;
    pub const GET_SCREEN_VIRTUAL_OFFSET: u32 = 0x40009;

    pub const SCREEN_PHY_RES: u32 = 0x48003;
    pub const SCREEN_VIRT_RES: u32 = 0x48004;
    pub const SET_SCREEN_DEPTH: u32 = 0x48005;
    pub const SET_SCREEN_ORDER: u32 = 0x48006;

    pub const SET_SCREEN_VIRT_OFF: u32 = 0x48009;

    pub const LAST: u32 = 0;
}

// Clocks
pub mod clock {
    pub const UART: u32 = 0x0_0000_0002;
}

// Responses
mod response {
    pub const SUCCESS: u32 = 0x8000_0000;
    pub const ERROR: u32 = 0x8000_0001; // error parsing request buffer (partial response)
}

pub const REQUEST: u32 = 0;

// Public interface to the mailbox
#[repr(C)]
#[repr(align(16))]
#[derive(Debug)]
pub struct Mbox<'a> {
    // The address for buffer needs to be 16-byte aligned so that the
    // Videcore can handle it properly.
    pub dma: &'a mut [u32],
    pub stack: [u32; 36],
    pub base_addr: usize,
    pub is_dma: bool,
    pub pos: usize,
}

impl<'a> ops::Deref for Mbox<'a> {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl<'a> Mbox<'a> {
    pub fn new(base_addr: usize) -> Mbox<'a> {
        Mbox { dma: &mut [], stack: [0; 36], base_addr, is_dma: false, pos: 2 }
    }

    pub fn new_with_dma(base_addr: usize) -> Mbox<'a> {
        let ptr: &mut [u32] = DMA.alloc_slice_zeroed(36, mem::align_of::<u32>()).unwrap();
        Mbox { dma: ptr, stack: [0; 36], base_addr, is_dma: true, pos: 2 }
    }

    pub fn prepare(&mut self, request: u32, total_size: u32, request_size: u32, params: &[u32]) {
        self.set_and_inc(request);
        self.set_and_inc(total_size);
        self.set_and_inc(request_size);

        for pos in 0..total_size / 4 {
            if pos as usize >= params.len() {
                self.set_and_inc(0)
            } else {
                self.set_and_inc(params[pos as usize])
            }
        }
    }

    pub fn clear(&mut self) {
        for _i in 0..self.pos {
            self.set_and_inc(0);
        }
        self.pos = 2;
    }

    pub fn request(&mut self, channel: u32) -> Result<(), MboxError> {
        // set the last "command"
        self.set_and_inc(mbox::tag::LAST);
        self.set_at_pos((self.pos * 4) as u32, 0);
        self.set_at_pos(mbox::REQUEST, 1);
        self.call(if self.is_dma { self.dma } else { &self.stack }, channel)
    }

    /// TODO: should become private once request is implemented everywhere
    pub fn call(&self, buffer: &[u32], channel: u32) -> Result<(), MboxError> {
        // wait until we can write to the mailbox
        loop {
            if !self.STATUS.is_set(STATUS::FULL) {
                break;
            }
            unsafe { asm!("nop") };
        }
        let buf_ptr = buffer.as_ptr() as u32;
        // write the address of our message to the mailbox with channel identifier
        self.WRITE.set((buf_ptr & !0xF) | (channel & 0xF));

        // now wait for the response
        loop {
            // is there a response?
            loop {
                if !self.STATUS.is_set(STATUS::EMPTY) {
                    break;
                }

                unsafe { asm!("nop") };
            }

            let resp: u32 = self.READ.get();

            // is it a response to our message?
            if ((resp & 0xF) == channel) && ((resp & !0xF) == buf_ptr) {
                // is it a valid successful response?
                return match buffer[1] {
                    response::SUCCESS => Ok(()),
                    response::ERROR => Err(MboxError::ResponseError),
                    _ => Err(MboxError::UnknownError)
                    ,
                };
            }
        }
    }

    fn set_and_inc(&mut self, value: u32) -> () {
        let pos = self.inc();
        self.set_at_pos(value, pos)
    }

    fn set_at_pos(&mut self, value: u32, position: usize) -> () {
        if self.is_dma {
            self.dma[position] = value;
        } else {
            self.stack[position] = value;
        }
    }

    fn inc(&mut self) -> usize {
        let ret = self.pos;
        self.pos = self.pos + 1;
        ret
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}