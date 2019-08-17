use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use core::borrow::Borrow;
use console_traits::UnicodeConsole;

#[macro_use]
pub mod macros;
mod devices;
mod memory;

static mut UART : Uart = devices::hw::Uart::new(memory::map::physical::UART_BASE);

pub fn main() -> ! {

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::physical::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    let mut console : Console = Console::new(lfb);
    //
    console.write_string("Starting AoS...");

    loop {}
}

