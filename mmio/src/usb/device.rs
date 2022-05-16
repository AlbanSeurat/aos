
/*

/*--------------------------------------------------------------------------}
{ 	USB parent used mainly by internal routines (details of parent hub)		}
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__)) UsbParent {
	unsigned Number : 8;											// @0	Unique device number of our parent sometimes called address or id
	unsigned PortNumber : 8;										// @8	This is the port we are connected to on our parent hub
	unsigned reserved : 16;											// @16  Reserved 16 bits
};


/*--------------------------------------------------------------------------}
{  Our structure that hold details about any USB device we have detected    }
{--------------------------------------------------------------------------*/
struct UsbDevice {
	struct UsbParent ParentHub;						// Details of our parent hub
	struct UsbPipe Pipe0;							// Usb device pipe AKA pipe0
	struct UsbPipeControl PipeCtrl0;				// Usb device pipe control AKA pipectrl0
	struct UsbConfigControl Config;					// Usb config control
	uint8_t MaxInterface ALIGN4;					// Maxiumum interface in array (varies with config and usually a lot less than the max array size)
	struct UsbInterfaceDescriptor Interfaces[MaxInterfacesPerDevice] ALIGN4; // These are available interfaces on this device
	struct UsbEndpointDescriptor Endpoints[MaxInterfacesPerDevice][MaxEndpointsPerDevice] ALIGN4; // These are available endpoints on this device
	struct usb_device_descriptor Descriptor ALIGN4;	// Device descriptor it's accessed a bit so we have a copy to save USB bus ... align it for ARM7/8

	enum PayLoadType PayLoadId;						// Payload type being carried
	union {											// It can only be any of the different payloads
		struct HubDevice* HubPayload;				// If this is a USB gateway node of a hub this pointer will be set to the hub data which is about the ports
		struct HidDevice* HidPayload;				// If this node has a HID function this pointer will be to the HID payload
		struct MassStorageDevice* MassPayload;		// If this node has a MASS STORAGE function this pointer will be to the Mass Storage payload
	};
};

 */
use crate::usb::pipe::{Pipe, PIPE_CONTROL, PIPE};
use register::InMemoryRegister;
use crate::timer::SystemTimer;
use usb::host::channel::HostChannel;
use usb::host::channel::transfer_size::CHANNEL_TRANSFER_SIZE::PACKET_ID::Usb_Pid_Setup;
use crate::macros::any_as_u8_slice;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug)]
pub struct DeviceParent {
    number: u8,
    portNumber: u8,
    __reserved: u16
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug)]
pub struct UsbDevice {
    parentHub : DeviceParent,
    pipe0 : Pipe,
}

/*

/*--------------------------------------------------------------------------}
{	  Device Request structure (8 bytes) as per the USB 2.0 standard		}
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__)) UsbDeviceRequest {
	uint8_t Type;													// +0x0
	enum UsbDeviceRequestRequest {
		// USB requests
		GetStatus = 0,
		ClearFeature = 1,
		SetFeature = 3,
		SetAddress = 5,
		GetDescriptor = 6,
		SetDescriptor = 7,
		GetConfiguration = 8,
		SetConfiguration = 9,
		GetInterface = 10,
		SetInterface = 11,
		SynchFrame = 12,
		// HID requests
		GetReport = 1,
		GetIdle = 2,
		GetProtocol = 3,
		SetReport = 9,
		SetIdle = 10,
		SetProtocol = 11,
	} Request : 8;													// +0x1
	uint16_t Value;													// +0x2
	uint16_t Index;													// +0x4
	uint16_t Length;												// +0x6
};
 */

#[allow(non_snake_case)]
#[repr(C)]
pub struct UsbDeviceRequest {
    pub requestType: u8,
    pub requestName: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16
}

impl UsbDevice {

    pub fn new() -> Self {
        UsbDevice {
            parentHub: DeviceParent {
                number: 0xFF,
                portNumber: 0,
                __reserved: 0
            },
            pipe0: Pipe {
                o: InMemoryRegister::new(0),
                c: InMemoryRegister::new(0),
            },
        }
    }

    pub fn send_control_message(&self, timer: &'static SystemTimer, request: &UsbDeviceRequest, channel : &HostChannel) -> Result<(), &'static str> {

        self.pipe0.o.write(PIPE::NUMBER.val(0) + PIPE::PACKET_SIZE::Bits8 + PIPE::SPEED::Full);
        self.pipe0.c.write( PIPE_CONTROL::TRANSFER_TYPE::Control + PIPE_CONTROL::DIRECTION::OUT + PIPE_CONTROL::CHANNEL.val(0));

        channel.send_receive_buffer(timer, &self.pipe0, unsafe { any_as_u8_slice(request) }, Usb_Pid_Setup.value)
        /*
                uint32_t lastTransfer = 0;

                // LOG("Setup phase ");
                // Setup phase
                struct UsbPipeControl intPipeCtrl = pipectrl;					// Copy the pipe control (We want channel really)
                intPipeCtrl.Type = USB_TRANSFER_TYPE_CONTROL;					// Set pipe to control
                intPipeCtrl.Direction = USB_DIRECTION_OUT;						// Set pipe to out
                if ((result = HCDChannelTransfer(pipe, intPipeCtrl,
                                                 (uint8_t*)request, 8, USB_PID_SETUP)) != OK) {				// Send the 8 byte setup request packet
                    LOG("HCD: SETUP packet to device: %#x req: %#x req Type: %#x Speed: %i PacketSize: %i LowNode: %i LowPort: %i Error: %i\n",
                        pipe.Number, request->Request, request->Type, pipe.Speed, pipe.MaxSize, pipe.lowSpeedNodePoint, pipe.lowSpeedNodePort, result);// Some parameter issue
                    return OK;
                }
                */

    }
}