use cortex_a::regs::{CNTFRQ_EL0, CNTP_TVAL_EL0, CNTP_CTL_EL0};
use register::cpu::{RegisterReadOnly, RegisterReadWrite};
use core::time::Duration;
use crate::timer::TimerError::WaitTooLong;

const NS_PER_S: u64 = 1_000_000_000;

pub struct Timer {

}

pub enum TimerError {
    WaitTooLong
}

pub type Result<T> = ::core::result::Result<T, TimerError>;

impl Timer {

    pub fn sleep(&self, duration: Duration) -> Result<()>{

        // Instantly return on zero.
        if duration.as_nanos() == 0 {
            return Ok(());
        }

        let frq = CNTFRQ_EL0.get() as u64;
        let x = match frq.checked_mul(duration.as_nanos() as u64) {
            None => {
                return Err(WaitTooLong);
            }
            Some(val) => val,
        };
        let tval = x / NS_PER_S;

        CNTP_TVAL_EL0.set(tval as u32);

        // Kick off the counting.                       // Disable timer interrupt.
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::SET);

        // ISTATUS will be '1' when cval ticks have passed. Busy-check it.
        while !CNTP_CTL_EL0.matches_all(CNTP_CTL_EL0::ISTATUS::SET) {}

        // Disable counting again.
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);

        Ok(())
    }
}