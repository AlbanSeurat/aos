use cortex_a::regs::{CNTFRQ_EL0, CNTP_TVAL_EL0, CNTV_CTL_EL0, CNTV_TVAL_EL0, CNTP_CTL_EL0};
use register::cpu::{RegisterReadOnly, RegisterReadWrite};
use core::time::Duration;
use register::mmio::ReadWrite;
use core::ops;

const NS_PER_S: u64 = 1_000_000_000;


#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    CORE0_TIMER_IRQCNTL: ReadWrite<u32>,                   // 0x00
}

pub struct Timer {
}

impl Timer {

    pub fn sleep(duration: Duration) {

        // Instantly return on zero.
        if duration.as_nanos() == 0 {
            return;
        }

        let tval = Timer::durationTotTick(duration);

        CNTV_TVAL_EL0.set(tval as u32);

        // Kick off the counting.                       // Disable timer interrupt.
        CNTV_CTL_EL0.modify(CNTV_CTL_EL0::ENABLE::SET + CNTV_CTL_EL0::IMASK::SET);

        // ISTATUS will be '1' when cval ticks have passed. Busy-check it.
        while !CNTV_CTL_EL0.matches_all(CNTV_CTL_EL0::ISTATUS::SET) {}

        // Disable counting again.
        CNTV_CTL_EL0.modify(CNTV_CTL_EL0::ENABLE::CLEAR);
    }

    pub fn reset_counter(duration: Duration) {

        let tval = Timer::durationTotTick(duration);
        CNTP_TVAL_EL0.set(tval as u32);
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);

        let regs = 0x4000_0040 as *const RegisterBlock;

        unsafe { (*regs).CORE0_TIMER_IRQCNTL.set(0x08) };

    }

    fn durationTotTick(duration: Duration) -> u64 {
        let frq = CNTFRQ_EL0.get() as u64;
        return frq * duration.as_nanos() as u64 / NS_PER_S;
    }

}