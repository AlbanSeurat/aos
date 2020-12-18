use register::mmio::ReadWrite;
use core::ops;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    IRQ_BASIC_PENDING: ReadWrite<u32>,  // 0x00
    IRQ_PENDING_1: ReadWrite<u32>,      // 0x04
    IRQ_PENDING_2: ReadWrite<u32>,      // 0x08
    FIQ_CONTROL: ReadWrite<u32>,        // 0x0C
    pub ENABLE_IRQS_1: ReadWrite<u32>,  // 0x10
    ENABLE_IRQS_2: ReadWrite<u32>,      // 0x14
    ENABLE_BASIC_IRQS: ReadWrite<u32>,  // 0x18
    DISABLE_IRQS_1: ReadWrite<u32>,     // 0x1C
    DISABLE_IRQS_2: ReadWrite<u32>,     // 0x20
    DISABLE_BASIC_IRQS: ReadWrite<u32>, // 0x24
}

/// Public interface to the GPIO MMIO area
pub struct IRQ {
    base_addr: usize,
}

impl ops::Deref for IRQ {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl IRQ {
    pub fn new(base_addr: usize) -> IRQ {
        IRQ { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}