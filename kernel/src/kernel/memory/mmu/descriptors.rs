use register::register_bitfields;
use crate::kernel::memory::kernel_mem_range::{AttributeFields, Descriptor};
use crate::kernel::memory::mmu::mair;
use crate::kernel::memory::{get_kernel_virt_addr_properties, get_user_virt_addr_properties};

register_bitfields! {u64,
    // AArch64 Reference Manual page 2150
    STAGE1_DESCRIPTOR [
        /// Privileged execute-never
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Various address fields, depending on use case
        LVL2_OUTPUT_ADDR_4KiB    OFFSET(21) NUMBITS(27) [], // [47:21]
        NEXT_LVL_TABLE_ADDR_4KiB OFFSET(12) NUMBITS(36) [], // [47:12]

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

pub const FOUR_KIB: usize = 4 * 1024;
pub const FOUR_KIB_SHIFT: usize = 12; // log2(4 * 1024)

pub const TWO_MIB: usize = 2 * 1024 * 1024;
pub const TWO_MIB_SHIFT: usize = 21; // log2(2 * 1024 * 1024)

/// A descriptor pointing to the next page table.
pub struct TableDescriptor(register::FieldValue<u64, STAGE1_DESCRIPTOR::Register>);

impl TableDescriptor {
    pub fn new(next_lvl_table_addr: usize) -> Result<TableDescriptor, &'static str> {
        if next_lvl_table_addr % FOUR_KIB != 0 {
            return Err("TableDescriptor: Address is not 4 KiB aligned.");
        }

        let shifted = next_lvl_table_addr >> FOUR_KIB_SHIFT;

        Ok(TableDescriptor(
            STAGE1_DESCRIPTOR::VALID::True
                + STAGE1_DESCRIPTOR::TYPE::Table
                + STAGE1_DESCRIPTOR::NEXT_LVL_TABLE_ADDR_4KiB.val(shifted as u64),
        ))
    }

    pub fn value(&self) -> u64 {
        self.0.value
    }
}

/// A function that maps the generic memory range attributes to HW-specific
/// attributes of the MMU.
fn into_mmu_attributes(
    attribute_fields: AttributeFields,
) -> register::FieldValue<u64, STAGE1_DESCRIPTOR::Register> {
    use crate::kernel::memory::{AccessPermissions, MemAttributes};

    // Memory attributes
    let mut desc = match attribute_fields.mem_attributes {
        MemAttributes::CacheableDRAM => {
            STAGE1_DESCRIPTOR::SH::InnerShareable + STAGE1_DESCRIPTOR::AttrIndx.val(mair::NORMAL)
        }
        MemAttributes::Device => {
            STAGE1_DESCRIPTOR::SH::OuterShareable + STAGE1_DESCRIPTOR::AttrIndx.val(mair::DEVICE)
        }
    };

    // Access Permissions
    desc += match attribute_fields.acc_perms {
        AccessPermissions::ReadOnlyKernel => STAGE1_DESCRIPTOR::AP::RO_EL1,
        AccessPermissions::ReadWriteKernel => STAGE1_DESCRIPTOR::AP::RW_EL1,
        AccessPermissions::ReadOnlyUser => STAGE1_DESCRIPTOR::AP::RO_EL1_EL0,
        AccessPermissions::ReadWriteUser => STAGE1_DESCRIPTOR::AP::RW_EL1_EL0,
    };

    // Execute Never
    desc += if attribute_fields.execute_never {
        STAGE1_DESCRIPTOR::PXN::True
    } else {
        STAGE1_DESCRIPTOR::PXN::False
    };

    desc
}

/// A Level2 block descriptor with 2 MiB aperture.
///
/// The output points to physical memory.
pub struct Lvl2BlockDescriptor(register::FieldValue<u64, STAGE1_DESCRIPTOR::Register>);

impl Lvl2BlockDescriptor {
    pub fn new(
        output_addr: usize,
        attribute_fields: AttributeFields,
    ) -> Result<Lvl2BlockDescriptor, &'static str> {
        if output_addr % TWO_MIB != 0 {
            return Err("BlockDescriptor: Address is not 2 MiB aligned.");
        }

        let shifted = output_addr >> TWO_MIB_SHIFT;

        Ok(Lvl2BlockDescriptor(
            STAGE1_DESCRIPTOR::VALID::True
                + STAGE1_DESCRIPTOR::AF::True
                + into_mmu_attributes(attribute_fields)
                + STAGE1_DESCRIPTOR::TYPE::Block
                + STAGE1_DESCRIPTOR::LVL2_OUTPUT_ADDR_4KiB.val(shifted as u64),
        ))
    }

    pub fn value(&self) -> u64 {
        self.0.value
    }
}

/// A page descriptor with 4 KiB aperture.
///
/// The output points to physical memory.
pub struct PageDescriptor(register::FieldValue<u64, STAGE1_DESCRIPTOR::Register>);

impl PageDescriptor {
    pub fn new(
        output_addr: usize,
        attribute_fields: AttributeFields,
    ) -> Result<PageDescriptor, &'static str> {
        if output_addr % FOUR_KIB != 0 {
            return Err("PageDescriptor: Address is not 4 KiB aligned.");
        }

        let shifted = output_addr >> FOUR_KIB_SHIFT;

        Ok(PageDescriptor(
            STAGE1_DESCRIPTOR::VALID::True
                + STAGE1_DESCRIPTOR::AF::True
                + into_mmu_attributes(attribute_fields)
                + STAGE1_DESCRIPTOR::TYPE::Table
                + STAGE1_DESCRIPTOR::NEXT_LVL_TABLE_ADDR_4KiB.val(shifted as u64),
        ))
    }

    pub fn value(&self) -> u64 {
        self.0.value
    }
}

// TODO : Try to generify the two functions
pub fn kernel_4k_page_mapping(virt_addr: usize) -> Option<PageDescriptor> {

    let option= match get_kernel_virt_addr_properties(virt_addr) {
        Err(s) => panic!(s),
        Ok(i) => i,
    };

    if option.is_some() {
        let (output_addr, attribute_fields) = option.unwrap();
        let page_desc = match PageDescriptor::new(output_addr, attribute_fields) {
            Err(s) => panic!(s),
            Ok(desc) => desc,
        };
        return Some(page_desc)
    } else {
        return None
    }
}

// we can not pass descriptor reference (linker is using 0xFFFFFFFCxxxxx addresses)
pub fn kernel_2M_page_mapping(virt_addr: usize) -> Option<Lvl2BlockDescriptor> {

    let option= match get_kernel_virt_addr_properties(virt_addr) {
        Err(s) => return panic!(s),
        Ok(i) => i,
    };

    return mapDescriptorToBlock(option);
}

pub fn user_2M_page_mapping(desc : &Descriptor, virt_addr: usize) -> Option<Lvl2BlockDescriptor> {

    let option= match get_user_virt_addr_properties(desc, virt_addr) {
        Err(s) => return panic!(s),
        Ok(i) => i,
    };

    return mapDescriptorToBlock(option);
}

fn mapDescriptorToBlock(option: Option<(usize, AttributeFields)>) -> Option<Lvl2BlockDescriptor> {
    if option.is_some() {
        let (output_addr, attribute_fields) = option.unwrap();
        let page_desc = match Lvl2BlockDescriptor::new(output_addr, attribute_fields) {
            Err(s) => panic!(s),
            Ok(desc) => desc,
        };
        return Some(page_desc)
    } else {
        return None
    }
}




