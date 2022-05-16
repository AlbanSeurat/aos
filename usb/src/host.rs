mod channel;
mod config;
mod port;

use core::ops;
use tock_registers::registers::ReadWrite;

use crate::host::channel::HostChannel;
use crate::host::port::{HOST_PORT};
use crate::host::config::HOST_CONFIG;
use mmio::timer::SystemTimer;



/*

/*--------------------------------------------------------------------------}
{					DWC USB HOST REGISTER POINTERS						    }
{--------------------------------------------------------------------------*/
#define DWC_HOST_CONFIG				((volatile __attribute__((aligned(4))) struct HostConfig*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x400))
#define DWC_HOST_FRAMEINTERVAL		((volatile __attribute__((aligned(4))) struct HostFrameInterval*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x404))
#define DWC_HOST_FRAMECONTROL		((volatile __attribute__((aligned(4))) struct HostFrameControl*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x408))
#define DWC_HOST_FIFOSTATUS			((volatile __attribute__((aligned(4))) struct HostFifoStatus*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x410))
#define DWC_HOST_INTERRUPT			((volatile __attribute__((aligned(4))) uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x414))
#define DWC_HOST_INTERRUPTMASK		((volatile __attribute__((aligned(4))) uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x418))
#define DWC_HOST_FRAMELIST			((volatile __attribute__((aligned(4))) uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x41C))
#define DWC_HOST_PORT				((volatile __attribute__((aligned(4))) struct HostPort*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x440))
#define DWC_HOST_CHANNEL			((volatile __attribute__((aligned(4))) struct HostChannel*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x500))

 */


#[allow(non_snake_case)]
#[repr(C)]
pub struct UsbHostRegisterBlock {
    pub CONFIG: ReadWrite<u32, HOST_CONFIG::Register>,
    // 0x400
    pub __reserved1: [u64; 7],
    // 0x404
    pub PORT: ReadWrite<u32, HOST_PORT::Register>,
    // 0x440
    pub __reserved2: [u64; 23],
    pub CHANNELS: [HostChannel; 8],                    // 0x500 // TODO : replace hardcoded value (found in my own raspi ...)
}

pub struct HostDeviceController {
    base_addr: usize,
    timer: &'static SystemTimer,
}


impl ops::Deref for HostDeviceController {
    type Target = UsbHostRegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl HostDeviceController {
    pub const fn new(base_addr: usize, timer: &'static SystemTimer) -> Self {
        HostDeviceController {
            base_addr,
            timer,
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const UsbHostRegisterBlock {
        self.base_addr as *const _
    }

    pub(crate) fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}