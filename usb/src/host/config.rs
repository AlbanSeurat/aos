use tock_registers::register_bitfields;

/*


/*--------------------------------------------------------------------------}
{                          USB HOST CONFIG STRUCTURE					    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) HostConfig {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile unsigned ClockRate : 2;						// @0
			volatile bool FslsOnly : 1;								// @2
			volatile unsigned _reserved3_6 : 4;						// @3
			volatile unsigned en_32khz_susp : 1;					// @7
			volatile unsigned res_val_period : 8;					// @8
			volatile unsigned _reserved16_22 : 7;					// @16
			volatile bool EnableDmaDescriptor : 1;					// @23
			volatile unsigned FrameListEntries : 2;					// @24
			volatile bool PeriodicScheduleEnable : 1;				// @26
			volatile bool PeriodicScheduleStatus : 1;				// @27
			volatile unsigned reserved28_30 : 3;					// @28
			volatile bool mode_chg_time : 1;						// @31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};
 */

register_bitfields! {
    u32,

    pub HOST_CONFIG [

        ENABLE_DMA_DESCRIPTOR OFFSET(23) NUMBITS(1) [],

        FSLS_ONLY OFFSET(2) NUMBITS(1) [],

        CLOCK_RATE OFFSET(0) NUMBITS(2) [
            Clock30_60MHz = 0b00,
            Clock48MHz    = 0b01,
            Clock6MHz     = 0b10
        ]
    ]
}