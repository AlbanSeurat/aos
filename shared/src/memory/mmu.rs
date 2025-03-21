use core::arch::asm;
use aarch64_cpu::{asm::barrier, registers::*, asm};
use crate::memory::mapping::{Descriptor};
use crate::memory::mair;
use crate::memory::translate::{Granule512MiB, TranslationGranule};
use crate::memory::pages::FixedSizeTranslationTable;
use core::slice::Iter;
use mmio::print;

/// This constant is the power-of-two exponent that defines the virtual address space size.
///
/// Values tested and known to be working:
///   - 30 (1 GiB)
///   - 31 (2 GiB)
///   - 32 (4 GiB)
///   - 33 (8 GiB)
const ADDR_SPACE_SIZE_EXPONENT: usize = 31;
const NUM_LVL2_TABLES: usize = (1 << ADDR_SPACE_SIZE_EXPONENT) >> Granule512MiB::SHIFT;
const T0SZ: u64 = (64 - ADDR_SPACE_SIZE_EXPONENT) as u64;

static mut KERNEL_TABLES: ArchTranslationTable = ArchTranslationTable::new();
static mut USER_TABLES: ArchTranslationTable = ArchTranslationTable::new();

pub type ArchTranslationTable = FixedSizeTranslationTable<NUM_LVL2_TABLES>;

pub const VIRTUAL_ADDR_START: usize = usize::MAX << ADDR_SPACE_SIZE_EXPONENT;

pub fn init() -> Result<(), &'static str> {
    // Prepare the memory attribute indirection register.
    mair::init();

    // Configure various settings of stage 1 of the EL1 translation regime.
    let ips = ID_AA64MMFR0_EL1.read(ID_AA64MMFR0_EL1::PARange);
    TCR_EL1.write(
        TCR_EL1::TBI0::Ignored
            + TCR_EL1::TBI1::Ignored
            + TCR_EL1::IPS.val(ips)
            + TCR_EL1::TG0::KiB_64 // 64 KiB granule
            + TCR_EL1::TG1::KiB_64 // 64 KiB granule
            + TCR_EL1::SH0::Inner
            + TCR_EL1::SH1::Inner
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::EPD1::EnableTTBR1Walks
            + TCR_EL1::T0SZ.val(T0SZ)  // Start walks at level 2
            + TCR_EL1::T1SZ.val(T0SZ), // Start walks at level 2
    );

    // Switch the MMU on.
    //
    // First, force all previous changes to be seen before the MMU is enabled.
    barrier::isb(barrier::SY);

    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.write(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    // Force MMU init to complete before next instruction
    barrier::isb(barrier::SY);
    Ok(())
}

#[inline]
pub fn disable() {
    // Enable the MMU and turn on data and instruction caching.
    SCTLR_EL1.write(SCTLR_EL1::M::Disable + SCTLR_EL1::C::NonCacheable + SCTLR_EL1::I::NonCacheable);
}

pub fn setup_kernel_tables(descriptors: &[Descriptor]) -> Result<(), &'static str> {
    unsafe {
        KERNEL_TABLES.map_descriptors(&descriptors.iter());
        TTBR1_EL1.set_baddr(KERNEL_TABLES.phys_base_addr() as u64);
        memory_flush();
    }
    Ok(())
}

pub fn setup_user_tables(descriptors: &[Descriptor]) -> Result<(), &'static str> {
    unsafe {
        USER_TABLES.map_descriptors(&descriptors.iter());
        TTBR0_EL1.set_baddr(USER_TABLES.phys_base_addr() as u64);
        memory_flush();
    }
    Ok(())
}

pub fn setup_dyn_user_tables(descriptors: &Iter<Descriptor>, tables: &mut ArchTranslationTable, pid: u16) {
    tables.map_descriptors(descriptors);
    TTBR0_EL1.write(TTBR0_EL1::ASID.val(pid as u64) + TTBR0_EL1::BADDR.val(tables.phys_base_addr() as u64 >> 1));
    memory_flush();
    print!("MMU Program mapping at address {:#}: \n{}", tables.phys_base_addr(), tables);
}

pub fn switch_user_tables(pid: u16, base_addr : u64) {
    memory_flush();
    TTBR0_EL1.write(TTBR0_EL1::ASID.val(pid as u64) + TTBR0_EL1::BADDR.val(base_addr >> 1));
    memory_flush();
}

fn memory_flush() {
    barrier::dsb(barrier::ISHST);
    unsafe {
        asm!("TLBI VMALLE1IS");
    }
    barrier::dsb(barrier::ISH);
    barrier::isb(barrier::SY);
    for _i in 1..10 {
        asm::nop()
    }
}

pub unsafe fn kernel_tables() -> &'static ArchTranslationTable {
    &KERNEL_TABLES
}

pub unsafe fn user_tables() -> &'static ArchTranslationTable {
    &USER_TABLES
}