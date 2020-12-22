use register::{register_bitfields, InMemoryRegister, Field};
use crate::memory::mapping::{AttributeFields, MemAttributes, AccessPermissions};
use crate::memory::mair;
use core::{fmt, convert};
use core::fmt::{Formatter, Display};


// A table descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-15.
register_bitfields! {u64,
    pub STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor.
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

register_bitfields! {u64,
    // AArch64 Reference Manual page 2150
    pub STAGE1_DESCRIPTOR [
        /// Privileged execute-never
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3).
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        /// Access flag
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

pub trait TranslationGranule {
    const SIZE: usize;
    const MASK: usize = Self::SIZE - 1;
    const ALIGN: usize = !Self::MASK;
    const SHIFT: usize;
}

pub enum Granule512MiB {}

impl TranslationGranule for Granule512MiB {
    const SIZE: usize = 512 * 1024 * 1024;
    const SHIFT: usize = 29;
}

pub enum Granule64KiB {}

impl TranslationGranule for Granule64KiB {
    const SIZE: usize = 64 * 1024;
    const SHIFT: usize = 16;
}

/// A descriptor pointing to the next page table.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct TableDescriptor(pub u64);

impl TableDescriptor {

    /// Returns the valid bit.
    pub fn is_valid(&self) -> bool {
        InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(self.0)
            .is_set(STAGE1_TABLE_DESCRIPTOR::VALID)
    }

    pub fn get(&self, field : Field<u64, STAGE1_TABLE_DESCRIPTOR::Register>) -> u64 {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(self.0);
        return val.read(field);
    }
}

impl convert::From<usize> for TableDescriptor {
    fn from(next_lvl_table_addr: usize) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = next_lvl_table_addr >> Granule64KiB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::VALID::True
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB.val(shifted as u64),
        );
        TableDescriptor(val.get())
    }
}

impl Display for TableDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(self.0);
        f.write_fmt(format_args!(" table : {:08x} , type : {:b} valid : {:>5}",
                                 val.read(STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB) << Granule64KiB::SHIFT,
                                 val.read(STAGE1_TABLE_DESCRIPTOR::TYPE),
                                 val.is_set(STAGE1_TABLE_DESCRIPTOR::VALID)))
    }
}

/// A page descriptor with 4 KiB aperture.
///
/// The output points to physical memory.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PageDescriptor(pub u64);

impl PageDescriptor {
    /// Create an instance.
    pub fn new(output_addr: usize, attribute_fields: &AttributeFields) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_DESCRIPTOR::Register>::new(0);

        let shifted = output_addr as u64 >> Granule64KiB::SHIFT;
        val.write(
            STAGE1_DESCRIPTOR::VALID::True
                + STAGE1_DESCRIPTOR::AF::True
                + attribute_fields.clone().into()
                + STAGE1_DESCRIPTOR::TYPE::Table
                + STAGE1_DESCRIPTOR::OUTPUT_ADDR_64KiB.val(shifted),
        );

        Self(val.get())
    }

    /// Returns the valid bit.
    pub fn is_valid(&self) -> bool {
        InMemoryRegister::<u64, STAGE1_DESCRIPTOR::Register>::new(self.0)
            .is_set(STAGE1_DESCRIPTOR::VALID)
    }

    pub fn addr(&self) -> u64 {
        let val = InMemoryRegister::<u64, STAGE1_DESCRIPTOR::Register>::new(self.0);
        return val.read(STAGE1_DESCRIPTOR::OUTPUT_ADDR_64KiB);
    }
}

/// Convert the kernel's generic memory attributes to HW-specific attributes of the MMU.
impl convert::From<AttributeFields> for register::FieldValue<u64, STAGE1_DESCRIPTOR::Register>
{
    fn from(attribute_fields: AttributeFields) -> Self {
        // Memory attributes.
        let mut desc = match attribute_fields.mem_attributes {
            MemAttributes::CacheableDRAM => {
                STAGE1_DESCRIPTOR::SH::InnerShareable
                    + STAGE1_DESCRIPTOR::AttrIndx.val(mair::NORMAL)
            }
            MemAttributes::Device => {
                STAGE1_DESCRIPTOR::SH::OuterShareable
                    + STAGE1_DESCRIPTOR::AttrIndx.val(mair::DEVICE)
            }
        };

        // Access Permissions.
        desc += match attribute_fields.acc_perms {
            AccessPermissions::ReadOnlyKernel => STAGE1_DESCRIPTOR::AP::RO_EL1,
            AccessPermissions::ReadWriteKernel => STAGE1_DESCRIPTOR::AP::RW_EL1,
            AccessPermissions::ReadOnlyUser => STAGE1_DESCRIPTOR::AP::RO_EL1_EL0,
            AccessPermissions::ReadWriteUser => STAGE1_DESCRIPTOR::AP::RW_EL1_EL0,
        };

        // Execute Never.
        desc += if attribute_fields.execute_never {
            STAGE1_DESCRIPTOR::PXN::True
        } else {
            STAGE1_DESCRIPTOR::PXN::False
        };

        desc
    }
}


impl Display for PageDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = InMemoryRegister::<u64, STAGE1_DESCRIPTOR::Register>::new(self.0);
        f.write_fmt(format_args!(" {: >5} |  {: >5} | {:2b}        | {:2b}     | {:3b}   | {: >5}  | 0x{:08x} => 0x{:08x}",
                                 value.is_set(STAGE1_DESCRIPTOR::PXN),
                                 value.is_set(STAGE1_DESCRIPTOR::AF),
                                 value.read(STAGE1_DESCRIPTOR::SH),
                                 value.read(STAGE1_DESCRIPTOR::AP),
                                 value.read(STAGE1_DESCRIPTOR::AttrIndx),
                                 value.is_set(STAGE1_DESCRIPTOR::TYPE),
                                 value.read(STAGE1_DESCRIPTOR::OUTPUT_ADDR_64KiB) << Granule64KiB::SHIFT,
                                 value.read(STAGE1_DESCRIPTOR::OUTPUT_ADDR_64KiB) + 1 << Granule64KiB::SHIFT
        ))
    }
}