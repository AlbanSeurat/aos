pub mod descriptors;

/// System memory map.
#[allow(dead_code)]
#[rustfmt::skip]
pub mod map {
    pub const START:                   usize =             0x0000_0000;
    pub const END:                     usize =             0xFFFF_FFFF;

    pub mod physical {
        pub const KERNEL_HEAP_START:   usize =             0x3500_0000;
        pub const KERNEL_HEAP_END:     usize =             0x36FF_FFFF;

        pub const MMA_MEMORY_START:    usize =             0x3700_0000;
        pub const MMA_MEMORY_END:      usize =             0x38FF_FFFF;

        pub const KERN_START:          usize =             0x3900_0000;
        pub const KERN_END:            usize =             0x3AFF_FFFF;

        pub const KERN_STACK_START:    usize =             0x3AF8_0000;
        pub const KERN_STACK_END:      usize =             0x3AFF_FFFF;

        pub const GPU_BASE:            usize =             0x3B00_0000;
        pub const GPU_END:             usize =             0x3EFF_FFFF;

        pub const MMIO_BASE:           usize =             0x3F00_0000;
        pub const IRQ_BASE:            usize = MMIO_BASE + 0x0000_B200;
        pub const MBOX_BASE:           usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const MMIO_END:            usize =             0x3FFF_FFFF;
    }

    pub mod peripheral {
        pub const START:    usize =             0x4000_0000;
        pub const END:      usize =             0x4020_0000;
    }

    pub mod virt {
        use shared::memory::mmu::VIRTUAL_ADDR_START;

        pub const START:               usize =   VIRTUAL_ADDR_START;
        pub const KERNEL_HEAP_START:   usize =  START + super::physical::KERNEL_HEAP_START;
        pub const KERNEL_HEAP_END:     usize =  START + super::physical::KERNEL_HEAP_END;
        pub const MMA_MEMORY_START:    usize =  START + super::physical::MMA_MEMORY_START;
        pub const MMA_MEMORY_END:      usize =  START + super::physical::MMA_MEMORY_END;
        pub const KERN_START:          usize =  START + super::physical::KERN_START;
        pub const KERN_STACK_START:    usize =  START + super::physical::KERN_STACK_START;

        pub const MMIO_BASE:           usize =     START + 0x3F00_0000;
        pub const SYS_TIMER_BASE:      usize = MMIO_BASE + 0x0000_3000;
        pub const IRQ_BASE:            usize = MMIO_BASE + 0x0000_B200;
        pub const MBOX_BASE:           usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const USB_BASE:            usize = MMIO_BASE + 0x0098_0000;

        pub mod peripheral {
            pub const START:    usize =             super::START + 0x4000_0000;
            pub const END:      usize =             super::START + 0x4020_0000;
        }
    }
}
