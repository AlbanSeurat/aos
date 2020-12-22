use crate::{debugln, debug};
use register::mmio::ReadOnly;
use crate::bcm::{DeviceMemoryBlock, LOCAL_TIMER_CONTROL_STATUS, LOCAL_TIMER_IRQ_CLEAN_RELOAD};

const RESET_VALUE: u32 = 38_461_538;


pub struct LocalTimer {}

impl LocalTimer {
    pub fn setup(device : &DeviceMemoryBlock) {
        unsafe {
            (*device).LOCAL_TIMER_CONTROL_STATUS.write(
                LOCAL_TIMER_CONTROL_STATUS::RELOAD_VALUE.val(RESET_VALUE)
                    + LOCAL_TIMER_CONTROL_STATUS::TIMER_ENABLED::True + LOCAL_TIMER_CONTROL_STATUS::INTERRUPT_ENABLE::True);

            (*device).LOCAL_TIMER_INTERRUPT_ROUTING.set(0); // IRQ link to Core0
            (*device).CORE0_TIMER_IRQCNTL.set(1 << 1); // activate IRQ for local timer in the IRQ table
        };
    }

    pub fn reset(device : &DeviceMemoryBlock) {
        unsafe {
            (*device).LOCAL_TIMER_IRQ_CLEAN_RELOAD.write(LOCAL_TIMER_IRQ_CLEAN_RELOAD::INTERRUPT_FLAG_CLEAR::True); // reset flag for interrupt
        }
    }
}
