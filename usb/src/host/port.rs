use tock_registers::register_bitfields;
/*

/*--------------------------------------------------------------------------}
{                         USB HOST PORT STRUCTURE						    }
{--------------------------------------------------------------------------*/
/* Due to the inconsistent design of the bits in this register, sometime it requires  zeroing
   bits in the register before the write, so you do not unintentionally write 1's to them. */
#define HOSTPORTMASK  ~0x2E								// These are the funky bits on this register and we "NOT" them to make "AND" mask
struct __attribute__((__packed__, aligned(4))) HostPort {
	union {
		struct __attribute__((__packed__, aligned(1))) {
			volatile bool Connect : 1;								// @0
			volatile bool ConnectChanged : 1;						// @1
			volatile bool Enable : 1;								// @2
			volatile bool EnableChanged : 1;						// @3
			volatile bool OverCurrent : 1;							// @4
			volatile bool OverCurrentChanged : 1;					// @5
			volatile bool Resume : 1;								// @6
			volatile bool Suspend : 1;								// @7
			volatile bool Reset : 1;								// @8
			volatile unsigned _reserved9 : 1;						// @9
			volatile unsigned PortLineStatus : 2;					// @10
			volatile bool Power : 1;								// @12
			volatile unsigned TestControl : 4;						// @13
			volatile UsbSpeed Speed : 2;							// @17
			volatile unsigned _reserved19_31 : 13;					// @19-31
		};
		volatile uint32_t Raw32;									// Union to access all 32 bits as a uint32_t
	};
};

    /* #define DWHCI_HOST_PORT_DEFAULT_MASK			(  DWHCI_HOST_PORT_CONNECT_CHANGED \
    | DWHCI_HOST_PORT_ENABLE	   \
    | DWHCI_HOST_PORT_ENABLE_CHANGED  \
    | DWHCI_HOST_PORT_OVERCURRENT_CHANGED) */
 */

pub const HOST_PORT_MASK : u32 = !0x2E;

register_bitfields! {
    u32,

    pub HOST_PORT [

          SPEED OFFSET(17) NUMBITS(2) [
               High = 0b00,
               Full = 0b01,
               Low  = 0b10
          ],

          POWER OFFSET(12) NUMBITS(1) [],

          RESET OFFSET(8) NUMBITS(1) [],

          OVERCURRENT_CHANGED OFFSET(5) NUMBITS(1) [],

          ENABLED_CHANGED OFFSET(3) NUMBITS(1) [],

          ENABLED OFFSET(2) NUMBITS(1) [],

          CONNECT_CHANGED OFFSET(1) NUMBITS(1) [],

          CONNECT OFFSET(0) NUMBITS(1) []
    ]
}

