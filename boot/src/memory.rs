pub mod descriptors;

/// System memory map.
#[rustfmt::skip]
#[allow(dead_code)]
pub mod map {
    pub const START:                   usize =             0x0000_0000;
    pub const END:                     usize =             0xFFFF_FFFF;

    pub mod physical {
        pub const BOOT_START:          usize =             super::START;
        pub const BOOT_END:            usize =             0x0100_0000;

        pub const KERN_START:          usize =             0x3900_0000;
        pub const KERN_END:            usize =             0x3AFF_FFFF;

        pub const KERN_STACK_START:    usize =             0x3AF8_0000;
        pub const KERN_STACK_END:      usize =             0x3AFF_FFFF;

        pub const GPU_BASE:            usize =             0x3B00_0000;
        pub const GPU_END:             usize =             0x3EFF_FFFF;

        pub const MMIO_BASE:           usize =             0x3F00_0000;
        pub const IRQ_BASE:            usize = MMIO_BASE + 0x0000_B000;
        pub const MBOX_BASE:           usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const MMIO_END:            usize =             0x3FFF_FFFF;
    }

    pub mod virt {
        use shared::memory::mmu::VIRTUAL_ADDR_START;

        pub const START:               usize =  VIRTUAL_ADDR_START;
        pub const KERN_START:          usize =  START + super::physical::KERN_START;
        pub const KERN_STACK_START:    usize =  START + super::physical::KERN_STACK_START;
    }
}
