use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{					   USB CORE OTG CONTROL STRUCTURE					    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CoreOtgControl {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool sesreqscs : 1;							// @0
			volatile bool sesreq : 1;								// @1
			volatile bool vbvalidoven : 1;							// @2
			volatile bool vbvalidovval : 1;							// @3
			volatile bool avalidoven : 1;							// @4
			volatile bool avalidovval : 1;							// @5
			volatile bool bvalidoven : 1;							// @6
			volatile bool bvalidovval : 1;							// @7
			volatile bool hstnegscs : 1;							// @8
			volatile bool hnpreq : 1;								// @9
			volatile bool HostSetHnpEnable : 1;						// @10
			volatile bool devhnpen : 1;								// @11
			volatile unsigned _reserved12_15 : 4;					// @12-15
			volatile bool conidsts : 1;								// @16
			volatile unsigned dbnctime : 1;							// @17
			volatile bool ASessionValid : 1;						// @18
			volatile bool BSessionValid : 1;						// @19
			volatile unsigned OtgVersion : 1;						// @20
			volatile unsigned _reserved21 : 1;						// @21
			volatile unsigned multvalidbc : 5;						// @22-26
			volatile bool chirpen : 1;								// @27
			volatile unsigned _reserved28_31 : 4;					// @28-31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

 */


register_bitfields! {
    u32,

    pub CORE_OTG_CONFIG [

        OTG_VERSION   OFFSET(20) NUMBITS(1) [],

        CONID_STATUS  OFFSET(16) NUMBITS(1) [],

        HOST_SET_HNP_ENABLE OFFSET(10) NUMBITS(1) []
    ]

}