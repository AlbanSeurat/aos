use tock_registers::register_bitfields;

pub enum CoreFifoFlush {
    FlushNonPeriodic = 0,
    FlushPeriodic1 = 1,
    FlushPeriodic2 = 2,
    FlushPeriodic3 = 3,
    FlushPeriodic4 = 4,
    FlushPeriodic5 = 5,
    FlushPeriodic6 = 6,
    FlushPeriodic7 = 7,
    FlushPeriodic8 = 8,
    FlushPeriodic9 = 9,
    FlushPeriodic10 = 10,
    FlushPeriodic11 = 11,
    FlushPeriodic12 = 12,
    FlushPeriodic13 = 13,
    FlushPeriodic14 = 14,
    FlushPeriodic15 = 15,
    FlushAll = 16,
}


/*

/*--------------------------------------------------------------------------}
{							 USB CORE RESET STRUCTURE					    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CoreReset {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool CoreSoft : 1;								// @0
			volatile bool HclkSoft : 1;								// @1
			volatile bool HostFrameCounter : 1;						// @2
			volatile bool InTokenQueueFlush : 1;					// @3
			volatile bool ReceiveFifoFlush : 1;						// @4
			volatile bool TransmitFifoFlush : 1;					// @5
			volatile unsigned TransmitFifoFlushNumber : 5;			// @6
			volatile unsigned _reserved11_29 : 19;					// @11
			volatile bool DmaRequestSignal : 1;						// @30
			volatile bool AhbMasterIdle : 1;						// @31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};
 */


register_bitfields! {
    u32,

    pub CORE_RESET [

        AHB_MASTER_IDLE OFFSET(31) NUMBITS(1) [],

        TRANSMIT_FIFO_FLUSH_NUMBER OFFSET(6) NUMBITS(5) [],

        TRANSMIT_FIFO_FLUSH OFFSET(5) NUMBITS(1) [],
        RECEIVE_FIFO_FLUSH OFFSET(4) NUMBITS(1) [],

        HARDWARE_CLOCK_SOFT OFFSET(1) NUMBITS(1) [],
        CORE_SOFT OFFSET(0) NUMBITS(1) []
    ]

}