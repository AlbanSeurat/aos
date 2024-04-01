#![no_std]

#[macro_use] extern crate mmio;

pub use bus::UsbBus;
use mmio::timer::SystemTimer;
use ::core::time::Duration;

mod bus;
pub mod host;
pub mod core;
pub mod power;

fn wait_for<F>(timer: &'static SystemTimer, check: F, timeout : Duration)
               -> Result<(), &'static str> where F: Fn() -> bool {
    let mut tick = timer.tick_count();
    while !check() {
        if timer.tick_count() - tick > timeout {
            return Err("poll error");
        }
    }
    Ok(())
}