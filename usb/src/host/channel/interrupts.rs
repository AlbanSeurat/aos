use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{	      INTERRUPT BITS ON THE USB CHANNELS ON THE DESIGNWARE 2.0		    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) ChannelInterrupts {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool TransferComplete : 1;						// @0
			volatile bool Halt : 1;									// @1
			volatile bool AhbError : 1;								// @2
			volatile bool Stall : 1;								// @3
			volatile bool NegativeAcknowledgement : 1;				// @4
			volatile bool Acknowledgement : 1;						// @5
			volatile bool NotYet : 1;								// @6
			volatile bool TransactionError : 1;						// @7
			volatile bool BabbleError : 1;							// @8
			volatile bool FrameOverrun : 1;							// @9
			volatile bool DataToggleError : 1;						// @10
			volatile bool BufferNotAvailable : 1;					// @11
			volatile bool ExcessiveTransmission : 1;				// @12
			volatile bool FrameListRollover : 1;					// @13
			unsigned _reserved14_31 : 18;							// @14-31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};
*/

register_bitfields! {
    u32,

    pub CHANNEL_INTERRUPTS [

        DATA_TOGGLE_ERROR OFFSET(10) NUMBITS(1) [],

        FRAME_OVERRUN OFFSET(9) NUMBITS(1) [],
        BABBLE_ERROR OFFSET(8) NUMBITS(1) [],
        NOT_YET_READY OFFSET(6) NUMBITS(1) [],
        ACKNOWLEDGEMENT OFFSET(5) NUMBITS(1) [],
        NEGATIVE_ACK OFFSET(4) NUMBITS(1) [],
        STALL OFFSET(3) NUMBITS(1) [],
        AHB_ERROR OFFSET(2) NUMBITS(1) [],
        HALT OFFSET(1) NUMBITS(1) [],

        TRANSFERT_COMPLETE OFFSET(0) NUMBITS(1) []
    ]
}
