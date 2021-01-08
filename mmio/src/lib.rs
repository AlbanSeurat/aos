#![no_std]
#![feature(llvm_asm)]
#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]

#[macro_use]
extern crate num_derive;

use crate::logger::Logger;

pub mod io;

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
mod dma;
mod process;

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
pub use dma::MemoryRegion;
pub use process::handle::{Handle, HandleType};

pub static mut LOGGER: Logger = Logger::new();
pub static mut SCREEN: Logger = Logger::new();
pub static mut DMA : dma::DMAMemory = dma::DMAMemory::new();