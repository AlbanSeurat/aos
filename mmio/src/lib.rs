#![no_std]
#![feature(asm)]

use crate::logger::Logger;

mod delays;
mod gpio;
mod mbox;
mod uart;
mod logger;
pub mod macros;

pub static mut LOGGER: Logger = Logger::new();

pub use gpio::GPIO;
pub use mbox::Mbox;
pub use uart::Uart;