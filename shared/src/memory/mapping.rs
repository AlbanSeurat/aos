use core::ops::RangeInclusive;

#[derive(Copy, Clone, Debug)]
pub enum MemAttributes {
    UncacheableDRAM,
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone, Debug)]
pub enum AccessPermissions {
    ReadOnlyKernel,
    ReadWriteKernel,
    ReadOnlyUser, // include kernel
    ReadWriteUser, // include kernel
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
            acc_perms: AccessPermissions::ReadWriteKernel,
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
    pub map : Mapping,
}


