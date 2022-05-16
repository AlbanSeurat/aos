use tock_registers::register_bitfields;

/*
/*--------------------------------------------------------------------------}
{                USB HOST CHANNEL TRANSFER SIZE STRUCTURE				    }
{--------------------------------------------------------------------------*/
struct HostTransferSize {
    union {
    struct {
    unsigned size : 19;								// @0-18	Size of data to send or receive, in bytes and can be greater than maximum packet length
    unsigned packet_count : 10;						// @19-28   Number of packets left to transmit or maximum number of packets left to receive
    enum PacketId {
    USB_PID_DATA0 = 0,
    USB_PID_DATA1 = 2,
    USB_PID_DATA2 = 1,
    USB_PID_SETUP = 3,
    USB_MDATA = 3,
    } packet_id : 2;								// @29		Various packet phase ID
    unsigned do_ping : 1;							// @31
    } __packed;
    volatile uint32_t Raw32;							// Union to access all 32 bits as a uint32_t
};
} __packed;

*/


register_bitfields! {
    u32,

    pub CHANNEL_TRANSFER_SIZE [

        SIZE OFFSET(0) NUMBITS(19) [],

        PACKET_COUNT OFFSET(19) NUMBITS(10) [],

        PACKET_ID OFFSET(29) NUMBITS(2) [
            Usb_Pid_Data0 = 0b00,
            Usb_Pid_Data1 = 0b10,
            Usb_Pid_Data2 = 0b01,
            Usb_Pid_Setup = 0b11
            //Usb_Mdata     = 0b11
        ],

        DO_PING OFFSET(31) NUMBITS(1) []

    ]
}
