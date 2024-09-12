use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{					DWC POWER AND CLOCK REGISTER STRUCTURE				    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) PowerReg {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool StopPClock : 1;							// @0
			volatile bool GateHClock : 1;							// @1
			volatile bool PowerClamp : 1;							// @2
			volatile bool PowerDownModules : 1;						// @3
			volatile bool PhySuspended : 1;							// @4
			volatile bool EnableSleepClockGating : 1;				// @5
			volatile bool PhySleeping : 1;							// @6
			volatile bool DeepSleep : 1;							// @7
			volatile unsigned _reserved8_31 : 24;					// @8-31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

 */


register_bitfields! {
    u32,
    pub POWER_REGISTER [
        STOP_PCLOCK OFFSET(0) NUMBITS(1) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct UsbPowerRegisterBlock {
    pub pw: ReadWrite<u32, POWER_REGISTER::Register>,
}

use core::ops;
use tock_registers::registers::ReadWrite;

pub struct UsbPower {
    base_addr: usize
}

impl ops::Deref for UsbPower {
    type Target = UsbPowerRegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl UsbPower {
    pub const fn new(base_addr: usize) -> Self {
        UsbPower {
            base_addr
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const UsbPowerRegisterBlock {
        self.base_addr as *const _
    }
}