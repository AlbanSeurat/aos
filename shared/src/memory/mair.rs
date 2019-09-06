use cortex_a::regs::*;

pub const DEVICE: u64 = 0;
pub const NORMAL: u64 = 1;

/// Setup function for the MAIR_EL1 register.
pub fn init() {
    // Define the memory types that we will map. Cacheable normal DRAM and
    // device.
    MAIR_EL1.write(
        // Attribute 1
        MAIR_EL1::Attr1_HIGH::Memory_OuterWriteBack_NonTransient_ReadAlloc_WriteAlloc
            + MAIR_EL1::Attr1_LOW_MEMORY::InnerWriteBack_NonTransient_ReadAlloc_WriteAlloc

            // Attribute 0
            + MAIR_EL1::Attr0_HIGH::Device
            + MAIR_EL1::Attr0_LOW_DEVICE::Device_nGnRE,
    );
}
