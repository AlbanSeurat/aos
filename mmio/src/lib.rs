#![no_std]
#![feature(asm)]

use crate::logger::Logger;

mod delays;
mod gpio;
mod mbox;
mod uart;
pub mod syscall;
pub mod logger;
pub mod macros;

pub static mut LOGGER: Logger = Logger::new();

pub use gpio::GPIO;
pub use mbox::Mbox;
pub use uart::Uart;
pub use syscall::SysCall;
