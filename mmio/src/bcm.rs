use register::{
    mmio::ReadWrite,
    register_bitfields,
};
use core::ops;
use register::mmio::ReadOnly;

// Local Timer control status
register_bitfields! {
    u32,

    pub LOCAL_TIMER_CONTROL_STATUS [

        INTERRUPT_FLAG OFFSET(31) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        INTERRUPT_ENABLE OFFSET(29) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        TIMER_ENABLED  OFFSET(28) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        RELOAD_VALUE OFFSET(0) NUMBITS(28) [] // [0:27]
    ]
}

// Local Timer control status
register_bitfields! {
    u32,

    pub LOCAL_TIMER_IRQ_CLEAN_RELOAD [

        INTERRUPT_FLAG_CLEAR OFFSET(31) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        TIMER_RELOADED OFFSET(30) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}


#[allow(non_snake_case)]
#[repr(C)]
pub struct DeviceMemoryBlock {
    __reserved_0: [u32; 3],
    // 0x00
    pub GPU_INTERRUPTS_ROUTING: ReadWrite<u32>,
    // 0x0C
    __reserved_1: [u32; 5],
    pub LOCAL_TIMER_INTERRUPT_ROUTING: ReadWrite<u32>,
    // 0x24
    __reserved_2: [u32; 3],
    // 0x28
    pub LOCAL_TIMER_CONTROL_STATUS: ReadWrite<u32, LOCAL_TIMER_CONTROL_STATUS::Register>,
    // 0x34
    pub LOCAL_TIMER_IRQ_CLEAN_RELOAD: ReadWrite<u32, LOCAL_TIMER_IRQ_CLEAN_RELOAD::Register>,
    // 0x38
    __reserved_3: u32,
    pub CORE0_TIMER_IRQCNTL: ReadWrite<u32>,
    // 0x40
    __reserved_4: [u32; 7],
    pub CORE0_INTERRUPT_SOURCE: ReadOnly<u32>,
    // 0x60
}


/// Public interface to the GPIO MMIO area
pub struct BCMDeviceMemory {
    base_addr: usize,
}

impl ops::Deref for BCMDeviceMemory {
    type Target = DeviceMemoryBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl BCMDeviceMemory {
    pub const fn new(base_addr: usize) -> BCMDeviceMemory {
        BCMDeviceMemory { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const DeviceMemoryBlock {
        self.base_addr as *const _
    }
}