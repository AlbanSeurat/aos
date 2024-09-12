use tock_registers::registers::{ReadWrite, ReadOnly};
use core::ops;
use core::time::Duration;
use tock_registers::interfaces::Readable;

#[allow(non_snake_case)]
#[repr(C)]
pub struct SystemTimerMemoryBlock {

    CONTROL_STATUS : ReadWrite<u32>,         // 0x00
    TIMER_LOW      : ReadOnly<u32>,		     // 0x04
    TIMER_HIGH     : ReadOnly<u32>,          // 0x08
    COMPARE_0      : ReadWrite<u32>,	     // 0x0C
    COMPARE_1      : ReadWrite<u32>,		 // 0x10
    COMPARE_2      : ReadWrite<u32>,		 // 0x14
    COMPARE_3      : ReadWrite<u32>,         // 0x18
}


pub struct SystemTimer {
    base_addr: usize,
}


impl ops::Deref for SystemTimer {
    type Target = SystemTimerMemoryBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}


impl SystemTimer {
    pub const fn new(base_addr: usize) -> Self {
        SystemTimer {
            base_addr
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const SystemTimerMemoryBlock {
        self.base_addr as *const _
    }


    pub fn tick_count(&self) -> Duration {
        return Duration::from_micros((self.TIMER_HIGH.get() as u64) << 32u64 | (self.TIMER_LOW.get() as u64));
    }

    pub fn wait(&self, time: Duration) {
        let tick = self.tick_count();
        while self.tick_count() - tick < time {}
    }
}
