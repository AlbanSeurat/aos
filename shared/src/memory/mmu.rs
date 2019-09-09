use cortex_a::{barrier, regs::*};
use crate::memory::mapping::{AttributeFields, Descriptor};
use crate::memory::mair;
use crate::memory::PageTable;
use crate::memory::pages;
use crate::memory::pages::{BaseAddr, TranslationTable};

pub fn init(descriptors: &[Descriptor], tables_base_addr : usize) -> Result<(), &'static str> {
    // Prepare the memory attribute indirection register.
    mair::init();

    let base_addr = match new_tables(descriptors, tables_base_addr) {
        Ok(addr) => addr,
        Err(s) => return Err(s)
    };
    unsafe {
        // Point to the LVL2 table base address in TTBR0.
        TTBR0_EL1.set_baddr(base_addr);
        // Point to the LVL2 table base address in TTBR1.
        TTBR1_EL1.set_baddr(base_addr);
    }

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

pub fn reset_user_tables () {
    TTBR0_EL1.set_baddr(0);
}

pub fn setup_kernel_tables(descriptors: &[Descriptor], tables_base_addr : usize) -> Result<(()), &'static str> {
    let base_addr = match new_tables(descriptors, tables_base_addr) {
        Ok(addr) => addr,
        Err(s) => return Err(s)
    };
    unsafe {
        TTBR1_EL1.set_baddr(base_addr);
        TTBR0_EL1.set_baddr(base_addr);
        // Force MMU init to complete before next instruction
        barrier::isb(barrier::SY);
        Ok(())
    }
}

pub fn setup_user_tables(descriptors: &[Descriptor], tables_base_addr : usize) -> Result<(()), &'static str> {
    let base_addr = match new_tables(descriptors, tables_base_addr) {
        Ok(addr) => addr,
        Err(s) => return Err(s)
    };
    unsafe {
        TTBR0_EL1.set_baddr(base_addr);
        // Force MMU init to complete before next instruction
        barrier::isb(barrier::SY);
        Ok(())
    }
}

fn new_tables(descriptors: &[Descriptor], tables_base_addr : usize) -> Result<(u64), &'static str> {

    let mut tb = TranslationTable::new(tables_base_addr);
    let level2 = match tb.alloc_table() {
        Ok(table) => table,
        Err(s) => return Err(s)
    };
    debugln!("table {:p}", level2);
    unsafe {
        match pages::init(&mut tb, &mut *level2, descriptors) {
            Ok(_) => (),
            Err(s) => return Err(s)
        };
        Ok((*level2).entries.base_addr_u64())
    }
}