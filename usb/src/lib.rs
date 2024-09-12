#![no_std]

extern crate mmio;

pub mod bus;
mod core;
mod power;
mod host;

use mmio::timer::SystemTimer;
use ::core::time::Duration;


fn wait_for<F>(timer: &'static SystemTimer, check: F, timeout : Duration)
               -> Result<(), &'static str> where F: Fn() -> bool {
    let tick = timer.tick_count();
    while !check() {
        if timer.tick_count() - tick > timeout {
            return Err("poll error");
        }
    }
    Ok(())
}