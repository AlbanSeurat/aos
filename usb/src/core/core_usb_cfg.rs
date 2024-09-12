use tock_registers::register_bitfields;

/*

/*--------------------------------------------------------------------------}
{	USB CORE CONTROL STRUCTURE	.. CARE WRITE WHOLE REGISTER .. NO BIT OPS  }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) UsbControl {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile unsigned toutcal : 3;							// @0
			volatile bool PhyInterface : 1;							// @3
			volatile enum UMode {
				ULPI,
				UTMI,
			}  ModeSelect : 1;										// @4
			volatile bool fsintf : 1;								// @5
			volatile bool physel : 1;								// @6
			volatile bool ddrsel : 1;								// @7
			volatile bool SrpCapable : 1;							// @8
			volatile bool HnpCapable : 1;							// @9
			volatile unsigned usbtrdtim : 4;						// @10
			volatile unsigned reserved1 : 1;						// @14
			volatile bool phy_lpm_clk_sel : 1;						// @15
			volatile bool otgutmifssel : 1;							// @16
			volatile bool UlpiFsls : 1;								// @17
			volatile bool ulpi_auto_res : 1;						// @18
			volatile bool ulpi_clk_sus_m : 1;						// @19
			volatile bool UlpiDriveExternalVbus : 1;				// @20
			volatile bool ulpi_int_vbus_indicator : 1;				// @21
			volatile bool TsDlinePulseEnable : 1;					// @22
			volatile bool indicator_complement : 1;					// @23
			volatile bool indicator_pass_through : 1;				// @24
			volatile bool ulpi_int_prot_dis : 1;					// @25
			volatile bool ic_usb_capable : 1;						// @26
			volatile bool ic_traffic_pull_remove : 1;				// @27
			volatile bool tx_end_delay : 1;							// @28
			volatile bool force_host_mode : 1;						// @29
			volatile bool force_dev_mode : 1;						// @30
			volatile unsigned _reserved31 : 1;						// @31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

 */



register_bitfields! {
    u32,

    pub USB_CONFIG [

        TS_DLINE_PULSE_ENABLE OFFSET(22) NUMBITS(1) [],
        ULPI_DRIVE_EXTERNAL_VUS OFFSET(20) NUMBITS(1) [],

        ULPI_CLK_SUSP_M OFFSET(19) NUMBITS(1) [],
        ULPI_FSLS OFFSET(17) NUMBITS(1) [],

        HNP_CAPABLE OFFSET(9) NUMBITS(1) [],
        SRP_CAPABLE OFFSET(8) NUMBITS(1) [],

        MODE_SELECT OFFSET(4) NUMBITS(1) [
            ULPI = 0b1,
            UTMI = 0b0
        ],

        PHYSICAL_INTERFACE OFFSET(3) NUMBITS(1) [
            Width8bit = 0b0,
            Width16bit = 0b1
        ]
    ]

}
