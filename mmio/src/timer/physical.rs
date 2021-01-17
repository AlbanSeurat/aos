use cortex_a::regs::{CNTFRQ_EL0, CNTP_CTL_EL0, RegisterReadWrite, RegisterReadOnly, CNTP_TVAL_EL0};
use core::time::Duration;
use crate::{debugln, debug, IRQ};
use crate::bcm::DeviceMemoryBlock;

const NS_PER_S: u64 = 1_000_000_000;

pub struct PhysicalTimer {
    inc : Duration,
}

impl PhysicalTimer {

    pub const fn new(duration: Duration) -> Self {
        PhysicalTimer {
            inc: duration
        }
    }

    pub fn setup(&self, device: &DeviceMemoryBlock) {
        device.CORE0_TIMER_IRQCNTL.set(1 << 1); // activate IRQ for local timer in the IRQ table
        CNTP_TVAL_EL0.set(PhysicalTimer::duration(self.inc) as u64);
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
    }

    pub fn reset_counter(&self) {
        CNTP_TVAL_EL0.set(PhysicalTimer::duration(self.inc) as u64);
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
    }

    fn duration(duration: Duration) -> u64 {
        let frq = CNTFRQ_EL0.get() as u64;
        return frq * duration.as_nanos() as u64 / NS_PER_S;
    }

}
