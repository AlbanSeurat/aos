use tock_registers::register_bitfields;
/*

struct __attribute__((__packed__, aligned(4))) CoreHardware {
	struct __attribute__((__packed__, aligned(1))) {
		volatile const unsigned Direction0 : 2;						// @0
		volatile const unsigned Direction1 : 2;						// @2
		volatile const unsigned Direction2 : 2;						// @4
		volatile const unsigned Direction3 : 2;						// @6
		volatile const unsigned Direction4 : 2;						// @8
		volatile const unsigned Direction5 : 2;						// @10
		volatile const unsigned Direction6 : 2;						// @12
		volatile const unsigned Direction7 : 2;						// @14
		volatile const unsigned Direction8 : 2;						// @16
		volatile const unsigned Direction9 : 2;						// @18
		volatile const unsigned Direction10 : 2;					// @20
		volatile const unsigned Direction11 : 2;					// @22
		volatile const unsigned Direction12 : 2;					// @24
		volatile const unsigned Direction13 : 2;					// @26
		volatile const unsigned Direction14 : 2;					// @28
		volatile const unsigned Direction15 : 2;					// @30
		volatile const enum {
			HNP_SRP_CAPABLE,
			SRP_ONLY_CAPABLE,
			NO_HNP_SRP_CAPABLE,
			SRP_CAPABLE_DEVICE,
			NO_SRP_CAPABLE_DEVICE,
			SRP_CAPABLE_HOST,
			NO_SRP_CAPABLE_HOST,
		} OperatingMode : 3;										// @32-34
		volatile const enum {
			SlaveOnly,
			ExternalDma,
			InternalDma,
		} Architecture : 2;											// @35
		volatile bool PointToPoint : 1;								// @37
		volatile const enum {
			NotSupported,
			Utmi,
			Ulpi,
			UtmiUlpi,
		} HighSpeedPhysical : 2;									// @38-39
		volatile const enum {
			Physical0,
			Dedicated,
			Physical2,
			Physcial3,
		} FullSpeedPhysical : 2;									// @40-41
		volatile const unsigned DeviceEndPointCount : 4;			// @42
		volatile const unsigned HostChannelCount : 4;				// @46
		volatile const bool SupportsPeriodicEndpoints : 1;			// @50
		volatile const bool DynamicFifo : 1;						// @51
		volatile const bool multi_proc_int : 1;						// @52
		volatile const unsigned _reserver21 : 1;					// @53
		volatile const unsigned NonPeriodicQueueDepth : 2;			// @54
		volatile const unsigned HostPeriodicQueueDepth : 2;			// @56
		volatile const unsigned DeviceTokenQueueDepth : 5;			// @58
		volatile const bool EnableIcUsb : 1;						// @63
		volatile const unsigned TransferSizeControlWidth : 4;		// @64
		volatile const unsigned PacketSizeControlWidth : 3;			// @68
		volatile const bool otg_func : 1;							// @71
		volatile const bool I2c : 1;								// @72
		volatile const bool VendorControlInterface : 1;				// @73
		volatile const bool OptionalFeatures : 1;					// @74
		volatile const bool SynchronousResetType : 1;				// @75
		volatile const bool AdpSupport : 1;							// @76
		volatile const bool otg_enable_hsic : 1;					// @77
		volatile const bool bc_support : 1;							// @78
		volatile const bool LowPowerModeEnabled : 1;				// @79
		volatile const unsigned FifoDepth : 16;						// @80
		volatile const unsigned PeriodicInEndpointCount : 4;		// @96
		volatile const bool PowerOptimisation : 1;					// @100
		volatile const bool MinimumAhbFrequency : 1;				// @101
		volatile const bool PartialPowerOff : 1;					// @102
		volatile const unsigned _reserved103_109 : 7;				// @103
		volatile const enum {
			Width8bit,
			Width16bit,
			Width8or16bit,
		} UtmiPhysicalDataWidth : 2;								// @110
		volatile const unsigned ModeControlEndpointCount : 4;		// @112
		volatile const bool ValidFilterIddigEnabled : 1;			// @116
		volatile const bool VbusValidFilterEnabled : 1;				// @117
		volatile const bool ValidFilterAEnabled : 1;				// @118
		volatile const bool ValidFilterBEnabled : 1;				// @119
		volatile const bool SessionEndFilterEnabled : 1;			// @120
		volatile const bool ded_fifo_en : 1;						// @121
		volatile const unsigned InEndpointCount : 4;				// @122
		volatile const bool DmaDescription : 1;						// @126
		volatile const bool DmaDynamicDescription : 1;				// @127
	};
};

 */

register_bitfields! {
    u32,

    pub HCD_HARDWARE_CONFIG_2 [

        HOST_CHANNEL_COUNT OFFSET(14) NUMBITS(4) [],

        FULL_SPEED_PHYSICAL OFFSET(8) NUMBITS(2) [
            Physical0    = 0b00,
            Dedicated    = 0b01,
            Physical2    = 0b10,
            Physical3    = 0b11
        ],

        HIGH_SPEED_PHYSICAL OFFSET(6) NUMBITS(2) [
            NotSupported = 0b00,
			Utmi         = 0b01,
			Ulpi         = 0b10,
			UtmiUlpi     = 0b11
        ],

        POINT_TO_POINT OFFSET(5) NUMBITS(1) [],

        ARCHITECTURE OFFSET(3) NUMBITS(2) [
            SlaveOnly   = 0x00,
            ExternalDma = 0b01,
            InternalDma = 0b10
        ],

        OPERATING_MODE OFFSET(0) NUMBITS(3) [
            HNP_SRP_CAPABLE       = 0b000,
            SRP_ONLY_CAPABLE      = 0b001,
            NO_HNP_SRP_CAPABLE    = 0b010,
            SRP_CAPABLE_DEVICE    = 0b011,
            NO_SRP_CAPABLE_DEVICE = 0b100,
            SRP_CAPABLE_HOST      = 0b101,
            NO_SRP_CAPABLE_HOST   = 0b111
        ]

    ]
}

register_bitfields! {
    u32,


    pub HCD_HARDWARE_CONFIG_4 [


        DED_FIFO_EN OFFSET(25) NUMBITS(1) [],

        UTMI_PHYSICAL_DATA_WIDTH OFFSET(14) NUMBITS(2) [
            Width8bit      = 0b00,
			Width16bit     = 0b01,
			Width8or16bit  = 0b10
        ]

    ]
}