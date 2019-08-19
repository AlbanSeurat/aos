use crate::kernel::memory::kernel_mem_range::{AttributeFields, Translation, Descriptor, MemAttributes, AccessPermissions, Mapping};
use core::ops::RangeInclusive;

pub mod mmu;

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

    pub mod virt {

        pub const KERN_START:          usize =             0x3AE0_0000;
        pub const KERN_END:            usize =             0x3AFF_FFFF;

        pub const KERN_STACK_START:    usize =             0x3AF8_0000;
        pub const KERN_STACK_END:      usize =             0x3AFF_FFFF;

        pub const GPU_BASE:            usize =             0x3B00_0000;
        pub const GPU_END:             usize =             0x3EFF_FFFF;
    }
}

/// Types used for compiling the virtual memory layout of the kernel using
/// address ranges.
pub mod kernel_mem_range {
    use core::ops::RangeInclusive;

    #[derive(Copy, Clone, Debug)]
    pub enum MemAttributes {
        CacheableDRAM,
        Device,
    }

    #[derive(Copy, Clone, Debug)]
    pub enum AccessPermissions {
        ReadOnly,
        ReadWrite,
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Translation {
        Identity,
        Offset(usize),
    }

    #[derive(Copy, Clone, Debug)]
    pub struct AttributeFields {
        pub mem_attributes: MemAttributes,
        pub acc_perms: AccessPermissions,
        pub execute_never: bool,
    }

    impl Default for AttributeFields {
        fn default() -> AttributeFields {
            AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Mapping {
        pub translation: Translation,
        pub attribute_fields: AttributeFields,
    }

    #[derive(Debug)]
    pub struct Descriptor {
        pub virtual_range: fn() -> RangeInclusive<usize>,
        pub map : Option<Mapping>
    }
}

/// A virtual memory layout that is agnostic of the paging granularity that the
/// hardware MMU will use.
///
/// Contains only special ranges, aka anything that is _not_ normal cacheable
/// DRAM.
pub static KERNEL_VIRTUAL_LAYOUT: [Descriptor; 6] = [
    // Kernel code and RO data
    Descriptor {
        virtual_range: || {
            // Using the linker script, we ensure that the RO area is consecutive and 4
            // KiB aligned, and we export the boundaries via symbols:
            //
            // [__ro_start, __ro_end)
            extern "C" {
                // The inclusive start of the read-only area, aka the address of the
                // first byte of the area.
                static __ro_start: u64;

                // The exclusive end of the read-only area, aka the address of
                // the first byte _after_ the RO area.
                static __ro_end: u64;
            }

            unsafe {
                // Notice the subtraction to turn the exclusive end into an
                // inclusive end
                RangeInclusive::new(
                    &__ro_start as *const _ as usize,
                    &__ro_end as *const _ as usize - 1,
                )
            }
        },
        map : Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadOnly,
                execute_never: false,
            },
        })
    },
    // Kernel data and BSS
    Descriptor {
        virtual_range: || {
            extern "C" {
                static __ro_end: u64;
                static __bss_end: u64;
            }

            unsafe {
                RangeInclusive::new(
                    &__ro_end as *const _ as usize,
                    &__bss_end as *const _ as usize - 1,
                )
            }
        },
        map: Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            }
        }),
    },
    // No man's land
    Descriptor {
        virtual_range: || {
            extern "C" {
                static __bss_end: u64;
            }

            unsafe {
                RangeInclusive::new(
                    &__bss_end as *const _ as usize,
                    map::virt::KERN_STACK_START - 1
                )
            }
        },
        map: None
    },
    // Kernel stack
    Descriptor {
        virtual_range: || {
            RangeInclusive::new(map::virt::KERN_STACK_START, map::virt::KERN_STACK_END)
        },
        map: Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        }),
    },
    // GPU Ram
    Descriptor {
        virtual_range: || RangeInclusive::new(map::virt::GPU_BASE, map::virt::GPU_END),
        map: Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            }
        })
    },
    // Device MMIO
    Descriptor {
        virtual_range: || RangeInclusive::new(map::physical::MMIO_BASE, map::physical::MMIO_END),
        map: Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        })
    },
];

/// For a given virtual address, find and return the output address and
/// according attributes.
///
/// If the address is not covered in VIRTUAL_LAYOUT, return none and do not allow to translate the address
fn get_virt_addr_properties(virt_addr: usize) -> Result<Option<(usize, AttributeFields)>, &'static str> {
    if virt_addr > map::END {
        return Err("Address out of range.");
    }

    for i in KERNEL_VIRTUAL_LAYOUT.iter() {
        if (i.virtual_range)().contains(&virt_addr) {
            if i.map.is_some() {
                let mapping = i.map.unwrap();
                let output_addr = match mapping.translation {
                    Translation::Identity => virt_addr,
                    Translation::Offset(a) => a + (virt_addr - (i.virtual_range)().start()),
                };
                return Ok(Some((output_addr, mapping.attribute_fields)));
            }
        }
    }
    Ok(None)
}
