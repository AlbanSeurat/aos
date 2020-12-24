#![no_std]
#![feature(llvm_asm)]

use crate::logger::Logger;

mod delays;
mod gpio;
mod mbox;
mod uart;
mod irq;
mod dwhci;
pub mod timer;
pub mod syscall;
pub mod logger;
pub mod macros;
mod bcm;
mod console;

pub static mut LOGGER: Logger = Logger::new();

pub use gpio::GPIO;
pub use mbox::Mbox;
pub use uart::Uart;
pub use syscall::SysCall;
pub use dwhci::DWHCI;
pub use timer::VirtualTimer;
pub use timer::LocalTimer;
pub use irq::IRQ;
pub use bcm::BCMDeviceMemory;
pub use console::FrameBufferConsole;
