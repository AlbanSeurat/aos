use tock_registers::register_bitfields;

/*

/*--------------------------------------------------------------------------}
{                USB HOST CHANNEL SPLIT CONTROL STRUCTURE				    }
{--------------------------------------------------------------------------*/
struct HostChannelSplitControl {
	union {
		struct {
			unsigned port_address : 7;						// @0-6		0-based index of the port on the high-speed hub Transaction Translator occurs
			unsigned hub_address : 7;						// @7-13	USB device address of the high-speed hub that acts as Transaction Translator
			unsigned transaction_position : 2;				// @14-15	If we are processing split the transation position Begin=2,End=1,Middle=0,All=3
			unsigned complete_split : 1;					// @16		1 to complete a Split transaction, 0 = normal transaction
			unsigned _reserved : 14;						// @17-30
			unsigned split_enable : 1;						// @31		Set to 1 to enable Split Transactions
		} __packed;
		volatile uint32_t Raw32;							// Union to access all 32 bits as a uint32_t
	};
} __packed;

 */


register_bitfields! {
    u32,

    pub HOST_SPLIT_CONTROL [

        SPLIT_ENABLE OFFSET(31) NUMBITS(1) [],

        TRANSITION_COMPLETE OFFSET(14) NUMBITS(2) [],

        COMPLETE_SPLIT OFFSET(16) NUMBITS(1) [],

        HUB_ADDRESS OFFSET(7) NUMBITS(7) [],
        PORT_ADDRESS OFFSET(0) NUMBITS(7) []
    ]
}
