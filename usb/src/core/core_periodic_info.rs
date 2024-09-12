/*

/*--------------------------------------------------------------------------}
{				 USB CORE NON PERIODIC INFO STRUCTURE					    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CoreNonPeriodicInfo {
	volatile __attribute__((aligned(4))) struct FifoSize Size;		// +0x28
	volatile __attribute__((aligned(4))) const struct NonPeriodicFifoStatus Status;	// Read Only +0x2c
};

/*--------------------------------------------------------------------------}
{                       USB CORE PERIODIC INFO STRUCTURE				    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) CorePeriodicInfo {
	volatile __attribute__((aligned(4))) struct FifoSize HostSize;	// +0x100
	volatile __attribute__((aligned(4))) struct FifoSize DataSize[15];// +0x104
};

/*--------------------------------------------------------------------------}
{				 USB CORE NON PERIODIC FIFO STATUS STRUCTURE			    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) NonPeriodicFifoStatus {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile unsigned SpaceAvailable : 16;					// @0
			volatile unsigned QueueSpaceAvailable : 8;				// @16
			volatile unsigned Terminate : 1;						// @24
			volatile enum {
				InOut = 0,
				ZeroLengthOut = 1,
				PingCompleteSplit = 2,
				ChannelHalt = 3,
			} TokenType : 2;										// @25
			volatile unsigned Channel : 4;							// @27
			volatile unsigned Odd : 1;								// @31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

/*--------------------------------------------------------------------------}
{	   FIFOSIZE STRUCTURE .. THERE ARE A FEW OF THESE ON DESIGNWARE 2.0     }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) FifoSize {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile unsigned StartAddress : 16;					// @0
			volatile unsigned Depth : 16;							// @16
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

 */


use tock_registers::registers::{ReadOnly, ReadWrite};

#[allow(non_snake_case)]
#[repr(C)]
pub struct FifoSize {
    pub START_ADDR: ReadWrite<u16>,
    pub DEPTH: ReadWrite<u16>,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct NonPeriodicFifoSize {
    pub SIZE: FifoSize,
    pub STATUS: ReadOnly<u32>,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct PeriodicFifoSize {
    pub HOST_SIZE: FifoSize,
    pub DATA_SIZE: [FifoSize; 15],
}

