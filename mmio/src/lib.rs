#![no_std]
#![feature(llvm_asm)]

use crate::logger::Logger;

mod delays;
mod gpio;
mod mbox;
mod uart;
mod dwhci;
pub mod syscall;
pub mod logger;
pub mod macros;

pub static mut LOGGER: Logger = Logger::new();

pub use gpio::GPIO;
pub use mbox::Mbox;
pub use uart::Uart;
pub use syscall::SysCall;
pub use dwhci::DWHCI;
