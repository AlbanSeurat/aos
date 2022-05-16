use register::{register_bitfields, FieldValue, InMemoryRegister, TryFromValue};
use register::mmio::ReadWrite;
use crate::timer::SystemTimer;
use crate::usb::pipe::PIPE::PACKET_SIZE::Value::{Bits8, Bits16, Bits32, Bits64};
use usb::host::{HostDeviceController, channel::HostChannel};
use usb::host::channel::transfer_size::CHANNEL_TRANSFER_SIZE::PACKET_ID::Usb_Pid_Setup;
use core::fmt::{Debug, Formatter};
use core::fmt;

/*


/*--------------------------------------------------------------------------}
{ 	USB pipe our own special structure encompassing a pipe in the USB spec	}
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__)) UsbPipe {
	UsbPacketSize MaxSize : 2;										// @0		Maximum packet size
	UsbSpeed Speed : 2;												// @2		Speed of device
	unsigned EndPoint : 4;											// @4		Endpoint address
	unsigned Number : 8;											// @8		Unique device number sometimes called address or id
	unsigned _reserved : 2;											// @16-17
	unsigned lowSpeedNodePort : 7;									// @18-24		In low speed transfers it is port device is on closest parent high speed hub
	unsigned lowSpeedNodePoint : 7;									// @25-31	In low speed transfers it is closest parent high speed hub
};

/*--------------------------------------------------------------------------}
{ 			USB pipe control used mainly by internal routines				}
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__)) UsbPipeControl {
	unsigned _reserved : 14;										// @0-13
	enum usb_transfer_type	Type : 2;								// @14-15	Packet type
	unsigned Channel : 8;											// @16-23   Channel to use
	unsigned Direction : 1;											// @24		Direction  1=IN, 0=OUT
	unsigned _reserved1 : 7;										// @25-31
};

 */


register_bitfields! {
    u32,

    pub PIPE [
          LOW_SPEED_NODE_PORT OFFSET(25) NUMBITS(7) [],
          LOW_SPEED_NODE_POINT OFFSET(18) NUMBITS(7) [],
          NUMBER OFFSET(8) NUMBITS(1) [],
          ENDPOINT OFFSET(4) NUMBITS(4) [],
          SPEED OFFSET(2) NUMBITS(2) [
            High = 0b00,
            Full = 0b01,
            Low  = 0b10
          ],
          PACKET_SIZE OFFSET(0) NUMBITS(2) [
                Bits8 = 0b0,
                Bits16 = 0b01,
                Bits32 = 0b10,
                Bits64 = 0b11
          ]
    ]
}

register_bitfields! {
    u32,

    pub PIPE_CONTROL [

        DIRECTION OFFSET(24) NUMBITS(1) [
            OUT = 0b0,
            IN  = 0b1
        ],

        CHANNEL OFFSET(16) NUMBITS(8) [],

        TRANSFER_TYPE OFFSET(14) NUMBITS(2) [
            Control     = 0b00,
	        Isochronous = 0b01,
	        Bulk        = 0b10,
	        Interrupt   = 0b11
        ]

    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Pipe {
    pub o: InMemoryRegister<u32, PIPE::Register>,
    pub c: InMemoryRegister<u32, PIPE_CONTROL::Register>,
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("CONTROL: {:b}, PIPE: {:b}", self.o.get(), self.c.get()))
    }
}

impl Pipe {

    pub fn size(&self) -> u32 {
        match self.o.read_as_enum(PIPE::PACKET_SIZE) {
            Some(Bits8) => 8,
            Some(Bits16) => 16,
            Some(Bits32) => 32,
            Some(Bits64) => 63,
            _ => 8
        }
    }
}