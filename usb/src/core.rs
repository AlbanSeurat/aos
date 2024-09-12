pub(crate) mod core_ahb;
mod core_reset;
mod core_usb_cfg;

mod core_irq;
mod core_hard_config;
mod core_otg_ctrl;
mod core_periodic_info;

use core_ahb::CORE_AHB_CONFIG;
use core_reset::CORE_RESET;
use core_usb_cfg::USB_CONFIG;
use core_irq::CORE_INTERRUPT;
use core::ops;
use mmio::{debugln, debug, mbox};
use crate::core::core_periodic_info::{PeriodicFifoSize, NonPeriodicFifoSize};
use crate::core::core_otg_ctrl::CORE_OTG_CONFIG;
use mmio::timer::SystemTimer;
use crate::wait_for;
use core::time::Duration;
use tock_registers::fields::FieldValue;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite};
use crate::core::core_hard_config::{HCD_HARDWARE_CONFIG_4, HCD_HARDWARE_CONFIG_2};
use crate::core::core_hard_config::HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL::Value::{NotSupported, Utmi, Ulpi, UtmiUlpi};
use crate::core::core_hard_config::HCD_HARDWARE_CONFIG_2::ARCHITECTURE::Value::{ExternalDma, InternalDma, SlaveOnly};
use crate::core::core_hard_config::HCD_HARDWARE_CONFIG_2::OPERATING_MODE::Value::{HNP_SRP_CAPABLE,
                                                                                 NO_HNP_SRP_CAPABLE,
                                                                                 NO_SRP_CAPABLE_DEVICE,
                                                                                 NO_SRP_CAPABLE_HOST,
                                                                                 SRP_CAPABLE_DEVICE,
                                                                                 SRP_CAPABLE_HOST,
                                                                                 SRP_ONLY_CAPABLE};

/*

/*--------------------------------------------------------------------------}
{					 DWC USB CORE REGISTER POINTERS						    }
{--------------------------------------------------------------------------*/
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
pub struct UsbCoreRegisterBlock {
    pub OTG_CTRL: ReadWrite<u32, CORE_OTG_CONFIG::Register>,
    // 0x00
    pub OTG_INT: ReadWrite<u32>,
    // 0x04
    pub AHB_CFG: ReadWrite<u32, CORE_AHB_CONFIG::Register>,
    // 0x08
    pub USB_CFG: ReadWrite<u32, USB_CONFIG::Register>,
    // 0x0C
    pub RESET: ReadWrite<u32, CORE_RESET::Register>,
    // 0x10
    pub INT_STAT: ReadWrite<u32, CORE_INTERRUPT::Register>,
    // 0x14
    pub INT_MASK: ReadWrite<u32, CORE_INTERRUPT::Register>,
    // 0x18
    pub RX_STAT_RD: ReadOnly<u32>,
    // 0x1C
    pub RX_STAT_POP: ReadOnly<u32>,
    // 0x20
    pub RX_FIFO_SIZ: ReadWrite<u32>,
    // 0x24
    pub NPER_TX_FIFO_SIZ: NonPeriodicFifoSize,
    // 0x2C
    pub I2C_CTRL: ReadWrite<u32>,
    // 0x30
    pub PHY_VENDOR_CTRL: ReadWrite<u32>,
    // 0x34
    pub GPIO: ReadWrite<u32>,
    // 0x38
    pub USER_ID: ReadWrite<u32>,
    // 0x3C
    pub VENDOR_ID: ReadWrite<u32>,
    // 0x40
    pub HARDWARE_CFG1: ReadOnly<u32>,
    // 0x44
    pub HARDWARE_CFG2: ReadOnly<u32, HCD_HARDWARE_CONFIG_2::Register>,
    // 0x4C
    pub HARDWARE_CFG3: ReadOnly<u32>,
    // 0x48
    pub HARDWARE_CFG4: ReadOnly<u32, HCD_HARDWARE_CONFIG_4::Register>,
    // 0x4C

    pub __reserved: [u64; 21],
    // 0x54

    pub PER_TX_FIFO_SIZ: PeriodicFifoSize,
    // 0x100
}

/*

 static const char dwc2_driver_name[] = "dwc2";

+static const struct dwc2_core_params params_bcm2835 = {
+	.otg_cap			= 0,	/* HNP/SRP capable */
+	.otg_ver			= 0,	/* 1.3 */
+	.dma_enable			= 1,
+	.dma_desc_enable		= 0,
+	.speed				= 0,	/* High Speed */
+	.enable_dynamic_fifo		= 1,
+	.en_multiple_tx_fifo		= 1,
+	.host_rx_fifo_size		= 774,	/* 774 DWORDs */
+	.host_nperio_tx_fifo_size	= 256,	/* 256 DWORDs */
+	.host_perio_tx_fifo_size	= 512,	/* 512 DWORDs */
+	.max_transfer_size		= 65535,
+	.max_packet_count		= 511,
+	.host_channels			= 8,
+	.phy_type			= 1,	/* UTMI */
+	.phy_utmi_width			= 8,	/* 8 bits */
+	.phy_ulpi_ddr			= 0,	/* Single */
+	.phy_ulpi_ext_vbus		= 0,
+	.i2c_enable			= 0,
+	.ulpi_fs_ls			= 0,
+	.host_support_fs_ls_low_power	= 0,
+	.host_ls_low_power_phy_clk	= 0,	/* 48 MHz */
+	.ts_dline			= 0,
+	.reload_ctl			= 0,
+	.ahbcfg				= 0x10,
+};
+
 */


pub struct UsbCoreController {
    base_addr: usize,
    timer: &'static SystemTimer,
}

impl ops::Deref for UsbCoreController {
    type Target = UsbCoreRegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl UsbCoreController {
    pub const fn new(base_addr: usize, timer: &'static SystemTimer) -> Self {
        UsbCoreController {
            base_addr,
            timer,
        }
    }
    /// Returns a pointer to the register block
    fn ptr(&self) -> *const UsbCoreRegisterBlock {
        self.base_addr as *const _
    }


    pub fn is_otg(&self) -> bool {
        match self.HARDWARE_CFG2.read_as_enum(HCD_HARDWARE_CONFIG_2::OPERATING_MODE) {
            Some(HNP_SRP_CAPABLE) | Some(SRP_ONLY_CAPABLE) | Some(NO_HNP_SRP_CAPABLE) => true,
            _ => false
        }
    }

    pub fn is_device(&self) -> bool {
        match self.HARDWARE_CFG2.read_as_enum( HCD_HARDWARE_CONFIG_2::OPERATING_MODE) {
            Some(SRP_CAPABLE_DEVICE) | Some(NO_SRP_CAPABLE_DEVICE) => true,
            _ => false
        }
    }

    pub fn is_host(&self) -> bool {
        match self.HARDWARE_CFG2.read_as_enum(HCD_HARDWARE_CONFIG_2::OPERATING_MODE) {
            Some(SRP_CAPABLE_HOST) | Some(NO_SRP_CAPABLE_HOST) => true,
            _ => false
        }
    }

    pub fn is_host_mode(&self) -> bool {
        return self.INT_STAT.is_set(CORE_INTERRUPT::CURRENT_MODE);
    }


    pub fn power(&self, v_mbox: &mut mbox::Mbox) -> Result<(), &'static str> {
        v_mbox.clear();
        v_mbox.prepare(mbox::tag::SET_POWER_STATE, 8, 8, &[3, 0b11 /* on and wait */]);
        v_mbox.request(mbox::channel::PROP).expect("Can not start usb device");
        Ok(())
    }

    pub fn reset(&self) -> Result<(), &'static str> {
        self.RESET.write(CORE_RESET::CORE_SOFT::SET);

        wait_for(self.timer, || { self.RESET.matches_all(CORE_RESET::CORE_SOFT::CLEAR) }, Duration::from_millis(10000))
            .map_err(|_| { "reset failed" })?;

        wait_for(self.timer, || { self.RESET.matches_all(CORE_RESET::AHB_MASTER_IDLE::SET) }, Duration::from_millis(10000))
            .map_err(|_| { "AHN master idle state" })?;

        Ok(())
    }

    pub fn force_mode(&self) -> Result<(), &'static str> {


        Ok(())

        /*
        void dwc2_force_mode(struct dwc2_hsotg *hsotg, bool host)
{
	u32 gusbcfg;
	u32 set;
	u32 clear;

	dev_dbg(hsotg->dev, "Forcing mode to %s\n", host ? "host" : "device");

	/*
	 * Force mode has no effect if the hardware is not OTG.
	 */
	if (!dwc2_hw_is_otg(hsotg))
		return;

	/*
	 * If dr_mode is either peripheral or host only, there is no
	 * need to ever force the mode to the opposite mode.
	 */
	if (WARN_ON(host && hsotg->dr_mode == USB_DR_MODE_PERIPHERAL))
		return;

	if (WARN_ON(!host && hsotg->dr_mode == USB_DR_MODE_HOST))
		return;

	gusbcfg = dwc2_readl(hsotg, GUSBCFG);

	set = host ? GUSBCFG_FORCEHOSTMODE : GUSBCFG_FORCEDEVMODE;
	clear = host ? GUSBCFG_FORCEDEVMODE : GUSBCFG_FORCEHOSTMODE;

	gusbcfg &= ~clear;
	gusbcfg |= set;
	dwc2_writel(hsotg, gusbcfg, GUSBCFG);

	dwc2_wait_for_mode(hsotg, host);
	return;
}
         */
    }

    pub(crate) fn init(&self) -> Result<(), &'static str> {
        self.AHB_CFG.write(CORE_AHB_CONFIG::DMA_ENABLED::CLEAR);
        self.USB_CFG.write(USB_CONFIG::ULPI_DRIVE_EXTERNAL_VUS::CLEAR +
            USB_CONFIG::TS_DLINE_PULSE_ENABLE::CLEAR);

        self.init_phy()?;
        self.init_abh_config()?;
        self.init_usb_config()?;

        self.OTG_CTRL.write(CORE_OTG_CONFIG::OTG_VERSION::CLEAR);

        //self.enable_common_interrupts()?;

        if self.is_host_mode() {
            debugln!("Host Mode");
        } else {
            debugln!("device mode");
        }
        Ok(())
    }


    pub(crate) fn init_phy(&self) -> Result<(), &'static str> {

        // Assume we are using UTMI
        let size = self.HARDWARE_CFG4.read(HCD_HARDWARE_CONFIG_4::UTMI_PHYSICAL_DATA_WIDTH);
        debugln!("PHY TYPE DATA WIDTH : {:b}", size);

        let mut field_value: FieldValue<u32, USB_CONFIG::Register> = match self.HARDWARE_CFG2.read_as_enum(HCD_HARDWARE_CONFIG_2::HIGH_SPEED_PHYSICAL) {
            Some(Ulpi) => Err("Ulpi mode not supported"),
            Some(Utmi) | Some(UtmiUlpi) => Ok(USB_CONFIG::MODE_SELECT::UTMI + USB_CONFIG::PHYSICAL_INTERFACE::Width8bit),
            Some(NotSupported) | None => Err("FS PHY selected at HS!")
        }?;

        if self.HARDWARE_CFG4.matches_all(HCD_HARDWARE_CONFIG_4::UTMI_PHYSICAL_DATA_WIDTH::Width16bit) {
            field_value = field_value + USB_CONFIG::PHYSICAL_INTERFACE::Width16bit;
        }

        if !self.USB_CFG.matches_all(field_value) {
            debugln!("device change");
            self.USB_CFG.write(field_value);
            self.reset()?;
        }

        // no ulpi_fs_ls for BCM2835
        self.USB_CFG.write(USB_CONFIG::ULPI_FSLS::CLEAR + USB_CONFIG::ULPI_CLK_SUSP_M::CLEAR);

        Ok(())
    }

    pub(crate) fn init_abh_config(&self) -> Result<(), &'static str> {
        let field_value = match self.HARDWARE_CFG2.read_as_enum(HCD_HARDWARE_CONFIG_2::ARCHITECTURE) {
            Some(ExternalDma) => Err("External DMA unsupported"),
            Some(SlaveOnly) => Err("Slave Only mode Unsupported"),
            Some(InternalDma) => Ok(CORE_AHB_CONFIG::DMA_ENABLED::SET
                + CORE_AHB_CONFIG::INTERRUPT_ENABLED::SET
                + CORE_AHB_CONFIG::EMPTY_LEVEL::SET
                + CORE_AHB_CONFIG::PERIODIC_EMPTY_LEVEL::SET),
            None => Err("Unsupported Mode")
        }?;

        /* p->ahbcfg = 0x10;
            ahbcfg &= GAHBCFG_CTRL_MASK;
			ahbcfg |= hsotg->params.ahbcfg &
			~GAHBCFG_CTRL_MASK;
         */
        let mut ahbcfg = self.AHB_CFG.extract().bitand(field_value.value).get() | (0x10 & !field_value.value);
        if !self.HARDWARE_CFG2.matches_all(HCD_HARDWARE_CONFIG_2::ARCHITECTURE::SlaveOnly) {
            ahbcfg = ahbcfg | CORE_AHB_CONFIG::DMA_ENABLED::SET.value
        }

        self.AHB_CFG.set(ahbcfg);
        debugln!("setup AHB_CFG to {:x}, results {:x}", ahbcfg, self.AHB_CFG.get());
        Ok(())
    }

    pub(crate) fn init_usb_config(&self) -> Result<(), &'static str> {
        match self.HARDWARE_CFG2.read_as_enum(HCD_HARDWARE_CONFIG_2::OPERATING_MODE) {
            Some(HNP_SRP_CAPABLE) => self.USB_CFG
                .write(USB_CONFIG::HNP_CAPABLE::SET + USB_CONFIG::SRP_CAPABLE::SET),
            Some(SRP_ONLY_CAPABLE) | Some(SRP_CAPABLE_DEVICE) | Some(SRP_CAPABLE_HOST) => self.USB_CFG
                .write(USB_CONFIG::HNP_CAPABLE::CLEAR + USB_CONFIG::SRP_CAPABLE::SET),
            Some(NO_HNP_SRP_CAPABLE) | Some(NO_SRP_CAPABLE_DEVICE) | Some(NO_SRP_CAPABLE_HOST) | None => self.USB_CFG
                .write(USB_CONFIG::HNP_CAPABLE::CLEAR + USB_CONFIG::SRP_CAPABLE::CLEAR)
        }
        Ok(())
    }

    fn enable_common_interrupts(&self) -> Result<(), &'static str> {
        /*
        u32 intmsk;

    /* Clear any pending OTG Interrupts */
    dwc2_writel(hsotg, 0xffffffff, GOTGINT);

    /* Clear any pending interrupts */
    dwc2_writel(hsotg, 0xffffffff, GINTSTS);

    /* Enable the interrupts in the GINTMSK */
    intmsk = GINTSTS_MODEMIS | GINTSTS_OTGINT;

    if (!hsotg->params.host_dma)
        intmsk |= GINTSTS_RXFLVL;
    if (!hsotg->params.external_id_pin_ctl)
        intmsk |= GINTSTS_CONIDSTSCHNG;

    intmsk |= GINTSTS_WKUPINT | GINTSTS_USBSUSP |
          GINTSTS_SESSREQINT;

    if (dwc2_is_device_mode(hsotg) && hsotg->params.lpm)
        intmsk |= GINTSTS_LPMTRANRCVD;

    dwc2_writel(hsotg, intmsk, GINTMSK);
         */

        Err("Not Implemented")
    }
}
