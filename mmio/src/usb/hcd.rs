use register::mmio::{ReadWrite, ReadOnly};

/*
#define DWC_CORE_OTGCONTROL			((volatile __attribute__((aligned(4))) struct CoreOtgControl*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x00))
#define DWC_CORE_OTGINTERRUPT		((volatile __attribute__((aligned(4))) struct CoreOtgInterrupt*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x04))
#define DWC_CORE_AHB				((volatile __attribute__((aligned(4))) struct CoreAhb*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x08))
#define DWC_CORE_CONTROL			((volatile __attribute__((aligned(4))) struct UsbControl*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x0C))
#define DWC_CORE_RESET				((volatile __attribute__((aligned(4))) struct CoreReset*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x10))
#define DWC_CORE_INTERRUPT			((volatile __attribute__((aligned(4))) struct CoreInterrupts*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x14))
#define DWC_CORE_INTERRUPTMASK		((volatile __attribute__((aligned(4))) struct CoreInterrupts*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x18))
#define DWC_CORE_RECEIVESIZE		((volatile __attribute__((aligned(4))) uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x24))
#define DWC_CORE_NONPERIODICFIFO	((volatile __attribute__((aligned(4))) struct CoreNonPeriodicInfo*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x28))
#define DWC_CORE_USERID				((volatile __attribute__((aligned(4))) uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x3C))
#define DWC_CORE_VENDORID			((volatile __attribute__((aligned(4))) const uint32_t*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x40))
#define DWC_CORE_HARDWARE			((volatile __attribute__((aligned(4))) const struct CoreHardware*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x44))
#define DWC_CORE_PERIODICINFO		((volatile __attribute__((aligned(4))) struct CorePeriodicInfo*)(uintptr_t)(RPi_IO_Base_Addr + USB_CORE_OFFSET + 0x100))

 */

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub OTG_CTRL: ReadWrite<u32>,  // 0x00
    pub OTG_INT: ReadWrite<u32>,   // 0x04
    pub AHB_CFG: ReadWrite<u32>,   // 0x08
    pub USB_CFG: ReadWrite<u32>,   // 0x0C
    pub RESET: ReadWrite<u32>,     // 0x10
    pub INT_STAT: ReadWrite<u32>,  // 0x14
    pub INT_MASK: ReadWrite<u32>,  // 0x18
    pub RX_STAT_RD: ReadOnly<u32>, // 0x1C
    pub RX_STAT_POP: ReadOnly<u32>,// 0x20
    pub RX_FIFO_SIZ: ReadWrite<u32>,// 0x24
    pub NPER_TX_FIFO_SIZ: ReadWrite<u32>,// 0x28
    pub NPER_TX_STAT: ReadWrite<u32>,// 0x2C
    pub I2C_CTRL: ReadWrite<u32>,  // 0x30
    pub PHY_VENDOR_CTRL: ReadWrite<u32>, // 0x34
    pub GPIO: ReadWrite<u32>,      // 0x38
    pub USER_ID: ReadWrite<u32>,   // 0x3C
    pub VENDOR_ID: ReadWrite<u32>,  // 0x40
    pub HW_CFG1: ReadOnly<u32>,
    pub HW_CFG2: ReadOnly<u32>,
    pub HW_CFG3: ReadOnly<u32>,
    pub HW_CFG4: ReadOnly<u32>,
}


pub struct HostDeviceController {
    base_addr: usize
}

impl HostDeviceController {

    pub const fn new(base_addr: usize) -> Self {
        HostDeviceController {
            base_addr
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    pub fn init(&self) -> Result<(), &'static str>{
        /*if self.ptr().VENDOR_ID & 0xfffff000 != 0x4f542000 {
            Err("HCD: Hardware: Driver incompatible. Expected OT2.xxx (BCM2708x).")
        }*/
        Ok(())
    }
}