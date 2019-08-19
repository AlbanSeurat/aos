use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{SP, RegisterReadWrite};
use core::fmt::Write;
use crate::reset;
use crate::kernel::memory::KERNEL_VIRTUAL_LAYOUT;

#[macro_use]
pub mod macros;
mod devices;
mod memory;

static mut UART : Uart = devices::hw::Uart::new(memory::map::physical::UART_BASE);

pub fn main() -> ! {


    match unsafe { memory::mmu::init() } {
        Err(s) => {
            debugln!("MMU error: {}\n", s);
        }
        // The following write is already using the identity mapped
        // translation in the LVL2 table.
        Ok(()) => ()
    }

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::physical::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    let mut console : Console = Console::new(lfb);
    log!(console, "Starting AoS...");

    loop {}
}

