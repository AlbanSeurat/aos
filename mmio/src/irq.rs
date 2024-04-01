use tock_registers::registers::ReadWrite;
use core::ops;
use core::arch::asm;
use tock_registers::interfaces::Writeable;

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

/// Public interface to the IRQ area
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
    pub const fn new(base_addr: usize) -> IRQ {
        IRQ { base_addr }
    }

    pub unsafe fn enable() {
        asm!("msr daifclr, #2");
    }

    pub unsafe fn disable() {
        asm!("msr daifset, #2");
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    pub fn local_enable(&self, irq: u32) {
        self.ENABLE_IRQS_1.set(irq);
    }

    pub fn external_enable(&self, irq: u32) {
        self.ENABLE_IRQS_2.set(irq);
    }
}