
/// System memory map.
#[rustfmt::skip]
pub mod map {
    pub const START:                   usize =             0x0000_0000;
    pub const END:                     usize =             0x3FFF_FFFF;

    pub mod physical {
        pub const MMIO_BASE:           usize =             0x3F00_0000;
        pub const VIDEOCORE_MBOX_BASE: usize = MMIO_BASE + 0x0000_B880;
        pub const GPIO_BASE:           usize = MMIO_BASE + 0x0020_0000;
        pub const UART_BASE:           usize = MMIO_BASE + 0x0020_1000;
        pub const MMIO_END:            usize =             super::END;
    }

    /*pub mod virt {
        pub const KERN_STACK_START:    usize =             super::START;
        pub const KERN_STACK_END:      usize =             0x0007_FFFF;
    }*/
}