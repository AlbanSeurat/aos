use register::{mmio::ReadWrite, register_bitfields};
use core::ops;
use register::mmio::ReadOnly;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub OTG_CTRL: ReadWrite<u32>,  // 0x00
    pub OTG_INT: ReadWrite<u32>,   // 0x04
    pub AHB_CFG: ReadWrite<u32>,   // 0x08
    pub USB_CFG: ReadWrite<u32>,   // 0x0C
    pub RESET: ReadWrite<u32>,     // 0x10
    pub INT_STAT: ReadWrite<u32>,  // 0x14
    pub INT_MASK: ReadWrite<u32>,  // 0x18
    pub RX_STAT_RD: ReadOnly<u32>, // 0x1C
    pub RX_STAT_POP: ReadOnly<u32>,// 0x20
    pub RX_FIFO_SIZ: ReadWrite<u32>,// 0x24
    pub NPER_TX_FIFO_SIZ: ReadWrite<u32>,// 0x28
    pub NPER_TX_STAT: ReadWrite<u32>,// 0x2C
    pub I2C_CTRL: ReadWrite<u32>,  // 0x30
    pub PHY_VENDOR_CTRL: ReadWrite<u32>, // 0x34
    pub GPIO: ReadWrite<u32>,      // 0x38
    pub USER_ID: ReadWrite<u32>,   // 0x3C
    pub VENDOR_ID: ReadWrite<u32>,  // 0x40
    pub HW_CFG1: ReadOnly<u32>,
    pub HW_CFG2: ReadOnly<u32>,
    pub HW_CFG3: ReadOnly<u32>,
    pub HW_CFG4: ReadOnly<u32>,
}

pub enum DWHCIError {

}
pub type Result<T> = ::core::result::Result<T, DWHCIError>;

pub struct DWHCI {
    base_addr: usize,
}

impl ops::Deref for DWHCI {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}


impl DWHCI {
    pub fn new(base_addr: usize) -> DWHCI {
        DWHCI { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    pub fn init(&self) -> Result<()> {

        Ok(())
    }
}