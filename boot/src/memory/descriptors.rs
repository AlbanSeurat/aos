use core::ops::RangeInclusive;
use shared::memory::mapping::{Translation, Mapping, MemAttributes, Granule,
                              AccessPermissions, Descriptor, AttributeFields};

/// A virtual memory layout that is agnostic of the paging granularity that the
/// hardware MMU will use.
///
pub static KERNEL_VIRTUAL_LAYOUT: [Descriptor; 3] = [
    // Kernel code and RO data
    /*Descriptor {
        virtual_range: || {
            extern "C" {
                static __ro_start: u64;
                static __ro_end: u64;
            }
            unsafe {
                RangeInclusive::new(
                    &__ro_start as *const _ as usize,
                    &__ro_end as *const _ as usize - 1,
                )
            }
        },
        map : Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadOnlyKernel,
                execute_never: false,
            },
        },
        granule : Granule::Regular
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
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            }
        },
        granule : Granule::Regular
    },
    // Kernel stack
    Descriptor {
        virtual_range: || {
            RangeInclusive::new(super::map::physical::KERN_STACK_START, super::map::physical::KERN_STACK_END)
        },
        map: Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: true,
            },
        },
        granule : Granule::Regular
    },*/
    Descriptor {
        virtual_range: || RangeInclusive::new(super::map::START, super::map::physical::KERN_END),
        map : Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWriteKernel,
                execute_never: false,
            },
        },
        granule : Granule::BigPage
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
        granule : Granule::BigPage
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
        granule : Granule::BigPage
    },
];
