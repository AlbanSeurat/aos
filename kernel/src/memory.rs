pub mod descriptors;

/// System memory map.
#[rustfmt::skip]
pub mod map {
    pub const START:                   usize =             0x0000_0000;
    pub const END:                     usize =             0x3FFF_FFFF;

    pub mod physical {
        pub const PROG_START:          usize =             0x0020_0000;
        pub const PROG_END:            usize =             0x0040_0000;

        pub const USER_MMU_START:      usize =             0x3AA0_0000;
        pub const USER_MMU_END:        usize =             0x3ABF_FFFF;

        pub const KERN_MMU_START:      usize =             0x3AC0_0000;
        pub const KERN_MMU_END:        usize =             0x3ADF_FFFF;

        pub const KERN_START:          usize =             0x3AE0_0000;
        pub const KERN_END:            usize =             0x3AFF_FFFF;

        pub const KERN_STACK_START:    usize =             0x3AF8_0000;
        pub const KERN_STACK_END:      usize =             0x3AFF_FFFF;

        pub const GPU_BASE:            usize =             0x3B00_0000;
        pub const GPU_END:             usize =             0x3EFF_FFFF;

        pub const MMIO_BASE:           usize =             0x3F00_0000;
        pub const MBOX_BASE:           usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const MMIO_END:            usize =             super::END;

    }

    pub mod virt {
        pub const START:               usize =   0xFFFF_FFFF_C000_0000;
        pub const MMIO_BASE:           usize =     START + 0x3F00_0000;
        pub const MBOX_BASE:           usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const USB_BASE:            usize = MMIO_BASE + 0x0098_0000;
    }
}
