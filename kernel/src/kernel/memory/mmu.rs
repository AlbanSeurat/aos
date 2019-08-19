use cortex_a::{barrier, regs::*};
use crate::kernel::memory::mmu::mair::set_up_mair;
use crate::kernel::memory::mmu::descriptors::{Lvl2BlockDescriptor, TWO_MIB_SHIFT, FOUR_KIB_SHIFT, PageDescriptor, TableDescriptor, get_block_mapping, get_page_mapping};
use crate::kernel::memory::{get_virt_addr_properties, AttributeFields, map};
use crate::kernel::memory::map::virt::{KERN_START, KERN_END};

mod mair;
mod descriptors;

trait BaseAddr {
    fn base_addr_u64(&self) -> u64;
    fn base_addr_usize(&self) -> usize;
}

impl BaseAddr for [u64; 512] {
    fn base_addr_u64(&self) -> u64 {
        self as *const u64 as u64
    }

    fn base_addr_usize(&self) -> usize {
        self as *const u64 as usize
    }
}

const NUM_ENTRIES_4KIB: usize = 512;

// A wrapper struct is needed here so that the align attribute can be used.
#[repr(C)]
#[repr(align(4096))]
struct PageTable {
    entries: [u64; NUM_ENTRIES_4KIB],
}

/// The LVL2 page table containng the 2 MiB entries.
static mut LVL2_TABLE: PageTable = PageTable {
    entries: [0; NUM_ENTRIES_4KIB],
};

/// The LVL3 page table containing the 4 KiB entries.
///
/// The kernel entry of the LVL2_TABLE will forward to this table.
static mut LVL3_TABLE: PageTable = PageTable {
    entries: [0; NUM_ENTRIES_4KIB],
};

unsafe fn setup_kernel() -> Result<(), &'static str>{
    const KERNEL_BLOCK_DESC: usize = map::virt::KERN_START >> TWO_MIB_SHIFT;

    // Point 2 MiB of virtual addresses to the follow-up LVL3
    // page-table.
    LVL2_TABLE.entries[KERNEL_BLOCK_DESC] = match TableDescriptor::new(LVL3_TABLE.entries.base_addr_usize()) {
        Err(s) => return Err(s),
        Ok(d) => d.value(),
    };

    for (page_descriptor_nr, entry) in LVL3_TABLE.entries.iter_mut().enumerate() {
        let virt_addr = page_descriptor_nr << FOUR_KIB_SHIFT;

        let option = get_page_mapping(KERN_START + virt_addr);
        if option.is_some() {
            *entry = option.unwrap().value();
        }
    }

    Ok(())
}

/// Setup MMU (only map kernel memory, gpu memory and MMIO
///
pub unsafe fn init() -> Result<(), &'static str> {
    // Prepare the memory attribute indirection register.
    set_up_mair();

    match setup_kernel() {
        Err(s) => return Err(s),
        Ok(i) => i
    }

    for (block_descriptor_nr, entry) in LVL2_TABLE.entries.iter_mut().enumerate().skip((map::virt::KERN_START >> TWO_MIB_SHIFT) + 1) {
        let virt_addr = block_descriptor_nr << TWO_MIB_SHIFT;

        let option = get_block_mapping(virt_addr);
        if option.is_some() {
            *entry = option.unwrap().value();
        }
    }

    // Point to the LVL2 table base address in TTBR0.
    TTBR0_EL1.set_baddr(LVL2_TABLE.entries.base_addr_u64());

    // Configure various settings of stage 1 of the EL1 translation regime.
    let ips = ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::PARange);
    TCR_EL1.write(
        TCR_EL1::TBI0::Ignored
            + TCR_EL1::IPS.val(ips)
            + TCR_EL1::TG0::KiB_4 // 4 KiB granule
            + TCR_EL1::SH0::Inner
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::T0SZ.val(34), // Start walks at level 2
    );

    // Switch the MMU on.
    //
    // First, force all previous changes to be seen before the MMU is enabled.
    barrier::isb(barrier::SY);

    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    // Force MMU init to complete before next instruction
    barrier::isb(barrier::SY);

    Ok(())
}