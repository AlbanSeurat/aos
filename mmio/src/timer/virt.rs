use cortex_a::regs::{CNTFRQ_EL0, CNTP_TVAL_EL0, CNTV_CTL_EL0, CNTV_TVAL_EL0, CNTP_CTL_EL0, RegisterReadWrite, RegisterReadOnly};
use core::time::Duration;

const NS_PER_S: u64 = 1_000_000_000;

pub struct VirtualTimer {
}

impl VirtualTimer {

    pub fn sleep(duration: Duration) {

        // Instantly return on zero.
        if duration.as_nanos() == 0 {
            return;
        }

        let tval = VirtualTimer::duration(duration);

        CNTV_TVAL_EL0.set(tval as u64);

        // Kick off the counting.                       // Disable timer interrupt.
        CNTV_CTL_EL0.modify(CNTV_CTL_EL0::ENABLE::SET + CNTV_CTL_EL0::IMASK::SET);

        // ISTATUS will be '1' when cval ticks have passed. Busy-check it.
        while !CNTV_CTL_EL0.matches_all(CNTV_CTL_EL0::ISTATUS::SET) {}

        // Disable counting again.
        CNTV_CTL_EL0.modify(CNTV_CTL_EL0::ENABLE::CLEAR);
    }

    pub fn reset_counter(duration: Duration) {
        let tval = VirtualTimer::duration(duration);
        CNTP_TVAL_EL0.set(tval as u64);
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);

    }

    fn duration(duration: Duration) -> u64 {
        let frq = CNTFRQ_EL0.get() as u64;
        return frq * duration.as_nanos() as u64 / NS_PER_S;
    }

}
