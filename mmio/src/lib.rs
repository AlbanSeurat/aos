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
pub mod timer;
pub mod syscall;
pub mod logger;
pub mod macros;
mod bcm;
mod console;
mod dma;
mod usb;

pub use gpio::GPIO;
pub use mbox::Mbox;
pub use uart::Uart;
pub use syscall::SysCall;
pub use usb::USB;
pub use timer::PhysicalTimer;
pub use irq::IRQ;
pub use bcm::BCMDeviceMemory;
pub use console::FrameBufferConsole;
use linked_list_allocator::LockedHeap;

pub static mut LOGGER: Logger = Logger::new();
pub static mut SCREEN: Logger = Logger::new();
pub static DMA: LockedHeap = LockedHeap::empty();

#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::empty();
