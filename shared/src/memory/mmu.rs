use cortex_a::{barrier, regs::*};
use crate::memory::mapping::{AttributeFields, Descriptor};
use crate::memory::mair;
use crate::memory::PageTable;
use crate::memory::pages::map_descriptor;
use crate::memory::pages::BaseAddr;

pub struct TranslationTable {
    tables_base_addr: usize,
    tables_count: usize,
}

impl TranslationTable {
    pub fn new(tables_base_addr: usize) -> TranslationTable {
        TranslationTable {
            tables_base_addr,
            tables_count: 0
        }
    }

    pub fn alloc_table(&mut self) -> Result<*mut PageTable, &'static str> {
        if self.tables_count < 512 {
            let page_addr = self.tables_base_addr + self.tables_count * 0x1000;
            self.tables_count += 1;
            unsafe {
                return Ok(page_addr as *mut PageTable);
            }
        } else {
            return Err("Can not allocate mmu descriptors table");
        }
    }
}
pub fn init(descriptors: &[Descriptor], page2: &mut PageTable) -> Result<(), &'static str> {
    // Prepare the memory attribute indirection register.
    mair::init();

    for desc in descriptors.iter() {
        map_descriptor(desc, page2);
    }

    debugln!("Assign LVL2 table : {:#x?}", page2.entries.base_addr_u64());

    // Point to the LVL2 table base address in TTBR0.
    TTBR0_EL1.set_baddr(page2.entries.base_addr_u64());
    // Point to the LVL2 table base address in TTBR1.
    TTBR1_EL1.set_baddr(page2.entries.base_addr_u64());

    // Configure various settings of stage 1 of the EL1 translation regime.
    let ips = ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::PARange);
    TCR_EL1.write(
        TCR_EL1::TBI0::Ignored
            + TCR_EL1::TBI1::Ignored
            + TCR_EL1::IPS.val(ips)
            + TCR_EL1::TG0::KiB_4 // 4 KiB granule
            + TCR_EL1::TG1::KiB_4
            + TCR_EL1::SH0::Inner
            + TCR_EL1::SH1::Inner
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::EPD1::EnableTTBR1Walks
            + TCR_EL1::T0SZ.val(34)  // Start walks at level 2
            + TCR_EL1::T1SZ.val(34), // Start walks at level 2
    );

    // Switch the MMU on.
    //
    // First, force all previous changes to be seen before the MMU is enabled.
    unsafe{
        barrier::isb(barrier::SY);
    }

    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    // Force MMU init to complete before next instruction
    unsafe{
        barrier::isb(barrier::SY);
    }

    Ok(())
}