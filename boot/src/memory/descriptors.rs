use core::ops::RangeInclusive;
use shared::memory::mapping::{Translation, Mapping, MemAttributes,
                              AccessPermissions, Descriptor, AttributeFields};

/// A virtual memory layout that is agnostic of the paging granularity that the
/// hardware MMU will use.
///
pub static KERNEL_VIRTUAL_LAYOUT: [Descriptor; 4] = [
    //Boot Kernel
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::BOOT_START, super::map::physical::BOOT_END - 1),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: false,
            },
        },
    },
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::KERN_START, super::map::physical::KERN_END - 1),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: false,
            },
        },
    },
    // GPU Ram
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::GPU_BASE, super::map::physical::GPU_END - 1),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            },
        },
    },
    // Device MMIO
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::MMIO_BASE, super::map::physical::MMIO_END - 1),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            },
        },
    }
];
