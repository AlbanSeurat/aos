use aarch64_cpu::registers::*;

pub const DEVICE: u64               = 0;
pub const NORMAL: u64               = 1;
pub const NORMAL_NON_CACHABLE: u64  = 2;

/// Setup function for the MAIR_EL1 register.
pub fn init() {
    // Define the memory types that we will map. Cacheable normal DRAM and
    // device.
    MAIR_EL1.write(
        // Attribute 2
        MAIR_EL1::Attr2_Normal_Outer::NonCacheable
            + MAIR_EL1::Attr2_Normal_Inner::NonCacheable

            // Attribute 1
            + MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc

            // Attribute 0
            + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck
    );
}
