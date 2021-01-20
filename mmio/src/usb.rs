mod hcd;

use register::{mmio::ReadWrite};
use core::ops;
use register::mmio::ReadOnly;

pub enum USBError {

}

pub struct USB {
    base_addr: usize,
}

impl USB {
    pub const fn new(base_addr: usize) -> USB {
        USB { base_addr }
    }

    pub fn init(&self) -> Result<(), USBError> {

        Ok(())
    }
}