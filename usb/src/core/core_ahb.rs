use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{	 USB CORE AHB STRUCTURE ... CARE WRITE WHOLE REGISTER .. NO BIT OPS	    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CoreAhb {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool InterruptEnable : 1;						// @0
			volatile enum {
				Length4 = 0,
				Length3 = 1,
				Length2 = 2,
				Length1 = 3,
			} AxiBurstLength : 2;									// @1
			volatile unsigned _reserved3 : 1;						// @3
			volatile bool WaitForAxiWrites : 1;						// @4
			volatile bool DmaEnable : 1;							// @5
			volatile unsigned _reserved6 : 1;						// @6
			volatile enum EmptyLevel {
				Empty = 1,
				Half = 0,
			} TransferEmptyLevel : 1;								// @7
			volatile enum EmptyLevel PeriodicTransferEmptyLevel : 1;// @8
			volatile unsigned _reserved9_20 : 12;					// @9
			volatile bool remmemsupp : 1;							// @21
			volatile bool notialldmawrit : 1;						// @22
			volatile enum {
				Incremental = 0,
				Single = 1, // (default)
			} DmaRemainderMode : 1;									// @23
			volatile unsigned _reserved24_31 : 8;					// @24-31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};
 */

// The AHB is a single-channel, shared bus to communicate between device
register_bitfields! {
    u32,

    pub CORE_AHB_CONFIG [

        DMA_REMAINDER_MODE OFFSET(23) NUMBITS(1) [
            Incremental = 0b0,
            Single = 0b1
        ],
        PERIODIC_EMPTY_LEVEL OFFSET(8) NUMBITS(1) [],
        EMPTY_LEVEL OFFSET(7) NUMBITS(1) [],
        DMA_ENABLED OFFSET(5) NUMBITS(1) [],
        INTERRUPT_ENABLED OFFSET(0) NUMBITS(1) []

    ]
}
