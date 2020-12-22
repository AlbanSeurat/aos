use core::ops::RangeInclusive;
use shared::memory::mapping::{Translation, Mapping, MemAttributes,
                              AccessPermissions, Descriptor, AttributeFields};

/// A virtual memory layout that is agnostic of the paging granularity that the
/// hardware MMU will use.
///
pub static KERNEL_VIRTUAL_LAYOUT: [Descriptor; 5] = [
    //Kernel
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::KERN_START, super::map::physical::KERN_STACK_START - 1),
        map : Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: false,
            },
        },
    },
    //Stack Kernel
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::KERN_STACK_START, super::map::physical::KERN_STACK_END - 1),
        map : Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            },
        },
    },
    // GPU Ram
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::GPU_BASE, super::map::physical::GPU_END),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            }
        },
    },
    // Device MMIO
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::MMIO_BASE, super::map::physical::MMIO_END),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            },
        },
    },
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::peripheral::START, super::map::peripheral::END),
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            }
        },
    }
];

pub static PROGRAM_VIRTUAL_LAYOUT: [Descriptor; 1] = [
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::physical::PROG_START, super::map::physical::PROG_END - 1),
        map : Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteUser,
                execute_never: false,
            },
        },
    }
];