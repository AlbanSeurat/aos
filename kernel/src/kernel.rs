use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{SP, RegisterReadWrite};
use core::fmt::Write;

#[macro_use]
pub mod macros;
mod devices;
mod memory;

static mut UART : Uart = devices::hw::Uart::new(memory::map::physical::UART_BASE);

pub fn main() -> ! {

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::physical::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    debug!("framebuffer : {:x?}", &lfb);
    let mut console : Console = Console::new(lfb);


    log!(console, "Starting AoS...");

    let stack = SP.get();
    log!(console, "stack {:x}", stack);

    loop {}
}

