use crate::core::UsbCoreController;
use crate::host::HostDeviceController;
use crate::power::UsbPower;
use mmio::timer::SystemTimer;

use core::time::Duration;
use tock_registers::interfaces::Readable;
use mmio::{debugln,debug};
use crate::core::core_ahb::CORE_AHB_CONFIG;

pub struct UsbBus {
    core: UsbCoreController,
    host: HostDeviceController,
    power: UsbPower,
    timer: &'static SystemTimer,
}

/*

	p->host_rx_fifo_size = 774;
	p->max_transfer_size = 65535;
	p->max_packet_count = 511;
	p->ahbcfg = 0x10;

 */

impl UsbBus {
    pub const fn new(base_addr: usize, timer: &'static SystemTimer) -> Self {
        UsbBus {
            core: UsbCoreController::new(base_addr, timer),
            host: HostDeviceController::new(base_addr + 0x400, timer),
            power: UsbPower::new(base_addr + 0xE00),
            timer
        }
    }

    pub fn init(&self) -> Result<(), &'static str> {
        debugln!("USB vendor : {:x} ", self.core.VENDOR_ID.get());

        if self.core.VENDOR_ID.get() & 0xfffff000 != 0x4f542000 {
            return Err("HCD: Hardware: Driver incompatible. Expected OT2.xxx (BCM2708x).");
        }
        self.core.reset();
        //wait for being in host mode...
        self.timer.wait(Duration::from_millis(100));
        debugln!("core is host mode now : {}", self.core.is_host_mode()); // works!
        debugln!("IRQ global status : {}", self.core.AHB_CFG.read(CORE_AHB_CONFIG::DMA_ENABLED));

        self.core.init();
        self.host.init()
    }


}
