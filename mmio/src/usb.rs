

/*use cortex_a::asm;
use register::{mmio::ReadWrite};
use register::mmio::ReadOnly;

use usb::core::core_ahb::CORE_AHB_CONFIG;
use usb::core::core_hard_config::HCD_HARDWARE_CONFIG_2;
use usb::core::core_hard_config::HCD_HARDWARE_CONFIG_2::FULL_SPEED_PHYSICAL::Value::Dedicated;
use usb::core::core_hard_config::HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL::Value::Ulpi;
use usb::core::core_hard_config::HCD_HARDWARE_CONFIG_2::OPERATING_MODE::Value::{HNP_SRP_CAPABLE,
                                                                               NO_HNP_SRP_CAPABLE,
                                                                               NO_SRP_CAPABLE_DEVICE,
                                                                               NO_SRP_CAPABLE_HOST,
                                                                               SRP_CAPABLE_DEVICE,
                                                                               SRP_CAPABLE_HOST,
                                                                               SRP_ONLY_CAPABLE};
use usb::core::core_otg_ctrl::CORE_OTG_CONFIG;
use usb::core::core_reset::CORE_RESET;
use usb::core::core_reset::CoreFifoFlush::FlushAll;
use usb::core::core_usb_cfg::USB_CONTROL;
use usb::core::core_usb_cfg::USB_CONTROL::MODE_SELECT::ULPI;
use usb::core::UsbCoreController;
use usb::host::{HostDeviceController, UsbHostRegisterBlock};
use usb::host::config::HOST_CONFIG;
use usb::power::UsbPower;

use crate::{debug, debugln, mbox};
use crate::timer::SystemTimer;
pub use crate::usb::device::{UsbDevice, UsbDeviceRequest};

mod device;
mod pipe;


// TODO : use DMA ?
const RECEIVE_FIFO_SIZE: u32 = 20480;  /* 16 to 32768 */
const NON_PERIODIC_FIFO_SIZE: u32 = 20480; /* 16 to 32768 */
const PERIODIC_FIFO_SIZE: u32 = 20480; /* 16 to 32768 */

pub struct USB {
    timer: &'static SystemTimer,
    core: UsbCoreController,
    host: HostDeviceController,
    power: UsbPower,
    started: bool,
}

impl USB {
    pub const fn new(base_addr: usize, timer: &'static SystemTimer) -> USB {
        USB {
            timer,
            core: UsbCoreController::new(base_addr, timer),
            host: HostDeviceController::new(base_addr + 0x400, timer),
            power: UsbPower::new(base_addr + 0xE00),
            started: false,
        }
    }

    pub fn init(&self, v_mbox: &mut mbox::Mbox) -> Result<(), &'static str> {
        debugln!("USB vendor : {:x} ", self.core.VENDOR_ID.get());

        if self.core.VENDOR_ID.get() & 0xfffff000 != 0x4f542000 {
            return Err("HCD: Hardware: Driver incompatible. Expected OT2.xxx (BCM2708x).");
        }

        if !self.core.HARDWARE_CFG1.matches_all(HCD_HARDWARE_CONFIG_2::ARCHITECTURE::InternalDma) {
            return Err("HCD: Host architecture does not support Internal DMA");
        }

        if self.core.HARDWARE_CFG1.matches_all(HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL::NotSupported) {
            return Err("HCD: High speed physical unsupported");
        }
        self.core.AHB_CFG.write(CORE_AHB_CONFIG::INTERRUPT_ENABLED::CLEAR);
        // clear all interrupts mask
        self.core.INT_MASK.set(0);

        self.core.power(v_mbox).expect("USB Power failed");

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.core.USB_CFG.write(USB_CONTROL::ULPI_DRIVE_EXTERNAL_VUS::CLEAR +
            USB_CONTROL::TS_DLINE_PULSE_ENABLE::CLEAR);
        self.core.reset()?;

        if !self.started {
            // If physical interface hasn't been initialized
            debugln!("HCD: One time phy initialisation.");
            self.started = true;
            self.core.USB_CFG.write(USB_CONTROL::MODE_SELECT::UTMI + USB_CONTROL::PHYSICAL_INTERFACE::CLEAR);
            self.core.reset()?;
        }

        if self.core.HARDWARE_CFG1.matches_any(HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL::Ulpi) {
            debugln!("HCD: ULPI FSLS configuration: enabled.");
            self.core.USB_CFG.write(USB_CONTROL::ULPI_FSLS::SET + USB_CONTROL::ULPI_CLK_SUM_M::SET);
        } else {
            debugln!("HCD: ULPI FSLS configuration: disabled.");
            self.core.USB_CFG.write(USB_CONTROL::ULPI_FSLS::CLEAR + USB_CONTROL::ULPI_CLK_SUM_M::CLEAR);
        }
        self.core.AHB_CFG.write(CORE_AHB_CONFIG::DMA_ENABLED::SET
            + CORE_AHB_CONFIG::DMA_REMAINDER_MODE::Incremental);

        match self.core.HARDWARE_CFG1.read_as_enum(HCD_HARDWARE_CONFIG_2::OPERATING_MODE) {
            Some(HNP_SRP_CAPABLE) => self.core.USB_CFG
                .write(USB_CONTROL::HNP_CAPABLE::SET + USB_CONTROL::SRP_CAPABLE::SET),
            Some(SRP_ONLY_CAPABLE) | Some(SRP_CAPABLE_DEVICE) | Some(SRP_CAPABLE_HOST) => self.core.USB_CFG
                .write(USB_CONTROL::HNP_CAPABLE::CLEAR + USB_CONTROL::SRP_CAPABLE::SET),
            Some(NO_HNP_SRP_CAPABLE) | Some(NO_SRP_CAPABLE_DEVICE) | Some(NO_SRP_CAPABLE_HOST) | None => self.core.USB_CFG
                .write(USB_CONTROL::HNP_CAPABLE::CLEAR + USB_CONTROL::SRP_CAPABLE::CLEAR)
        }
        debugln!("HCD: Core started.");
        debugln!("HCD: Starting host.");

        self.power.pw.set(0);

        if self.core.HARDWARE_CFG1.matches_any(HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL::Ulpi)
            && self.core.HARDWARE_CFG1.matches_any(HCD_HARDWARE_CONFIG_2::FULL_SPEED_PHYSICAL::Dedicated) {
            debugln!("HCD: Host clock: 48Mhz.");
            self.host.CONFIG.write(HOST_CONFIG::CLOCK_RATE::Clock48MHz);
        } else {
            debugln!("HCD: Host clock: 30-60Mhz.");
            self.host.CONFIG.write(HOST_CONFIG::CLOCK_RATE::Clock30_60MHz);
        }

        self.host.CONFIG.write(HOST_CONFIG::FSLS_ONLY::SET);

        self.core.RX_FIFO_SIZ.set(RECEIVE_FIFO_SIZE);

        // TODO : START_ADDR SHOULD MAYBE NEED TO BE CHANGED ?
        self.core.NPER_TX_FIFO_SIZ.SIZE.START_ADDR.set(NON_PERIODIC_FIFO_SIZE as u16);
        self.core.NPER_TX_FIFO_SIZ.SIZE.DEPTH.set(RECEIVE_FIFO_SIZE as u16);

        // TODO : START_ADDR SHOULD MAYBE NEED TO BE CHANGED ?
        self.core.PER_TX_FIFO_SIZ.HOST_SIZE.START_ADDR.set((RECEIVE_FIFO_SIZE + NON_PERIODIC_FIFO_SIZE) as u16);
        self.core.PER_TX_FIFO_SIZ.HOST_SIZE.DEPTH.set(PERIODIC_FIFO_SIZE as u16);

        debugln!("HCD: Set HNP: enabled.");

        self.core.OTG_CTRL.write(CORE_OTG_CONFIG::HOST_SET_HNP_ENABLE::SET);
        self.core.flush_tx(FlushAll)?;
        self.core.flush_rx()?;

        if self.host.CONFIG.matches_all(HOST_CONFIG::ENABLE_DMA_DESCRIPTOR::CLEAR) {
            self.host.reset_channels(self.core.HARDWARE_CFG1.read(HCD_HARDWARE_CONFIG_2::HOST_CHANNEL_COUNT) as usize)?;
        }

        self.host.power()?;

        Ok(())
    }

    pub fn send_request(&self, root : &mut UsbDevice, request: &UsbDeviceRequest) -> Result<(), &'static str> {

        root.send_control_message(self.timer, request, &self.host.CHANNELS[0])

        /*

/*-INTERNAL: EnumerateDevice ------------------------------------------------
 This is called from USBInitialize and will allocate our fake rootHub device
 and then begin enumeration of the whole USB bus.
 11Feb17 LdB
 --------------------------------------------------------------------------*/
RESULT UsbAttachRootHub(void) {
	RESULT result;
	struct UsbDevice *rootHub = NULL;
	LOG_DEBUG("Allocating RootHub\n");
	if (DeviceTable[0].PayLoadId != 0)								// If RootHub is already in use
		UsbDeallocateDevice(&DeviceTable[0]);						// We will need to deallocate it and every child
	result = UsbAllocateDevice(&rootHub);							// Try allocating the root hub now
	if (rootHub != &DeviceTable[0]) result = ErrorCompiler;			// Somethign really wrong .. 1st allocation should always be DeviceList[0]
	if (result != OK) return result;								// Return error result somethging fatal happened
	DeviceTable[0].Pipe0.Speed = USB_SPEED_FULL;					// Set our fake hub to full speed .. as it's fake we cant really ask it speed can we :-)
	DeviceTable[0].Pipe0.MaxSize = Bits64;							// Set our fake hub to 64 byte packets .. as it's fake we need to do it manually
	DeviceTable[0].Config.Status = USB_STATUS_POWERED;				// Set our fake hub status to configured .. as it's fake we need to do manually
	RootHubDeviceNumber = 0;										// Roothub number is zero
	return EnumerateDevice(&DeviceTable[0], NULL, 0);				// Ok start enumerating the USB bus as roothub port 1 is the physical bus
}

         */
    }

}
*/