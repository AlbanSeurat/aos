use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{                USB HOST CHANNEL CHARACTERISTIC STRUCTURE				    }
{--------------------------------------------------------------------------*/
struct HostChannelCharacteristic {
	union {
		struct {
			unsigned max_packet_size : 11;					// @0-10	Maximum packet size the endpoint is capable of sending or receiving
			unsigned endpoint_number : 4;					// @11-14	Endpoint number (low 4 bits of bEndpointAddress)
			unsigned endpoint_direction : 1;				// @15		Endpoint direction 1=IN, 0=OUT
			unsigned _reserved : 1;							// @16
			unsigned low_speed : 1;							// @17		1 when the device being communicated with is at low speed, 0 otherwise
			unsigned endpoint_type : 2;						// @18-19	Endpoint type (low 2 bits of bmAttributes)
			unsigned packets_per_frame : 2;					// @20-21	Maximum number of transactions that can be executed per microframe
			unsigned device_address : 7;					// @22-28	USB device address of the device on which the endpoint is located
			unsigned odd_frame : 1;							// @29		Before enabling channel must be set to opposite of low bit of host_frame_number
			unsigned channel_disable : 1;					// @30		Software can set this to 1 to halt the channel
			unsigned channel_enable : 1;					// @31		Software can set this to 1 to enable the channel
		} __packed;
		volatile uint32_t Raw32;							// Union to access all 32 bits as a uint32_t
	};
} __packed;
 */

register_bitfields! {
    u32,

    pub HOST_CHANNEL_CHARACTERISTIC [

        CHANNEL_ENABLED OFFSET(31) NUMBITS(1) [],
        CHANNEL_DISABLED OFFSET(30) NUMBITS(1) [],
        DEVICE_ADDRESS OFFSET(22) NUMBITS(7) [],
        PACKETS_PER_FRAME OFFSET(20) NUMBITS(2) [],
        ENDPOINT_TYPE OFFSET(18) NUMBITS(2) [],
        LOW_SPEED OFFSET(17) NUMBITS(1) [],
        ENDPOINT_DIRECTION OFFSET(15) NUMBITS(1) [
            OUT = 0b0,
            IN  = 0b1
        ],
        ENDPOINT_NUMBER OFFSET(11) NUMBITS(4) [],
        MAX_PACKET_SIZE OFFSET(0) NUMBITS(11) []
    ]
}
