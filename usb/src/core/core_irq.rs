use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{	       INTERRUPT BITS ON THE USB CORE OF THE DESIGNWARE 2.0		        }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CoreInterrupts {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool CurrentMode : 1;							// @0
			volatile bool ModeMismatch : 1;							// @1
			volatile bool Otg : 1;									// @2
			volatile bool DmaStartOfFrame : 1;						// @3
			volatile bool ReceiveStatusLevel : 1;					// @4
			volatile bool NpTransmitFifoEmpty : 1;					// @5
			volatile bool ginnakeff : 1;							// @6
			volatile bool goutnakeff : 1;							// @7
			volatile bool ulpick : 1;								// @8
			volatile bool I2c : 1;									// @9
			volatile bool EarlySuspend : 1;							// @10
			volatile bool UsbSuspend : 1;							// @11
			volatile bool UsbReset : 1;								// @12
			volatile bool EnumerationDone : 1;						// @13
			volatile bool IsochronousOutDrop : 1;					// @14
			volatile bool eopframe : 1;								// @15
			volatile bool RestoreDone : 1;							// @16
			volatile bool EndPointMismatch : 1;						// @17
			volatile bool InEndPoint : 1;							// @18
			volatile bool OutEndPoint : 1;							// @19
			volatile bool IncompleteIsochronousIn : 1;				// @20
			volatile bool IncompleteIsochronousOut : 1;				// @21
			volatile bool fetsetup : 1;								// @22
			volatile bool ResetDetect : 1;							// @23
			volatile bool Port : 1;									// @24
			volatile bool HostChannel : 1;							// @25
			volatile bool HpTransmitFifoEmpty : 1;					// @26
			volatile bool LowPowerModeTransmitReceived : 1;			// @27
			volatile bool ConnectionIdStatusChange : 1;				// @28
			volatile bool Disconnect : 1;							// @29
			volatile bool SessionRequest : 1;						// @30
			volatile bool Wakeup : 1;								// @31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};
 */


register_bitfields! {
    u32,

    pub CORE_INTERRUPT [

        DISCONNECT OFFSET(3) NUMBITS(1) [],
        SESSION_REQUEST OFFSET(2) NUMBITS(1) [],
        WAKE_UP OFFSET(1) NUMBITS(1) [],
        CURRENT_MODE OFFSET(0) NUMBITS(1) [
            Device = 0b0,
            Host = 0b1
        ]
    ]

}