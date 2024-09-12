mod characteristic;
mod interrupts;
mod split;
mod transfer_size;

use tock_registers::{LocalRegisterCopy};
use crate::host::channel::HostChannelError::{RecoverableError, Timeout, FatalError};
use crate::host::channel::HostChannelAction::{Success, ResendSplit};
use crate::host::channel::characteristic::HOST_CHANNEL_CHARACTERISTIC;
use crate::host::channel::split::HOST_SPLIT_CONTROL;
use crate::host::channel::interrupts::CHANNEL_INTERRUPTS;
use crate::host::channel::transfer_size::CHANNEL_TRANSFER_SIZE;
use mmio::timer::SystemTimer;
use core::time::Duration;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::ReadWrite;

/*
/*--------------------------------------------------------------------------}
{					  USB HOST CHANNEL STRUCTURE						    }
{--------------------------------------------------------------------------*/
struct __attribute__((__packed__, aligned(4))) HostChannel {
	volatile __attribute__((aligned(4))) struct HostChannelCharacteristic Characteristic;	// +0x0
	volatile __attribute__((aligned(4))) struct HostChannelSplitControl SplitCtrl;			// +0x4
	volatile __attribute__((aligned(4))) struct ChannelInterrupts Interrupt;				// +0x8
	volatile __attribute__((aligned(4))) struct ChannelInterrupts InterruptMask;			// +0xc
	volatile __attribute__((aligned(4))) struct HostTransferSize TransferSize;				// +0x10
	volatile __attribute__((aligned(4))) uint32_t  DmaAddr;									// +0x14
	volatile __attribute__((aligned(4))) uint32_t _reserved18;								// +0x18
	volatile __attribute__((aligned(4))) uint32_t _reserved1c;								// +0x1c
};

 */
#[derive(Debug)]
pub enum HostChannelError {
    FatalError,
    RecoverableError,
    Timeout
}
#[derive(Debug)]
pub enum HostChannelAction {
    Success,
    ResendSplit,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct HostChannel {
    pub CHARACTERISTIC: ReadWrite<u32, HOST_CHANNEL_CHARACTERISTIC::Register>,
    pub SPLIT_CONTROL: ReadWrite<u32, HOST_SPLIT_CONTROL::Register>,
    pub INTERRUPT: ReadWrite<u32, CHANNEL_INTERRUPTS::Register>,
    pub INTERRUPT_MASK: ReadWrite<u32, CHANNEL_INTERRUPTS::Register>,
    pub TRANSFER_SIZE: ReadWrite<u32, CHANNEL_TRANSFER_SIZE::Register>,
    pub DMA_ADDR: ReadWrite<u32>,
}

impl HostChannel {
    pub fn reset(&self, timer: &'static SystemTimer) -> Result<(), &'static str> {
        self.CHARACTERISTIC
            .write(HOST_CHANNEL_CHARACTERISTIC::CHANNEL_ENABLED::CLEAR
                + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_DISABLED::SET
                + HOST_CHANNEL_CHARACTERISTIC::ENDPOINT_DIRECTION::IN
            );
        self.CHARACTERISTIC
            .write(HOST_CHANNEL_CHARACTERISTIC::CHANNEL_ENABLED::SET
                + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_DISABLED::SET
                + HOST_CHANNEL_CHARACTERISTIC::ENDPOINT_DIRECTION::IN
            );

        let tick = timer.tick_count();
        while self.CHARACTERISTIC
            .matches_all(HOST_CHANNEL_CHARACTERISTIC::CHANNEL_ENABLED::SET) {
            if timer.tick_count() - tick > Duration::from_micros(100000) {
                return Err("HCD: Unable to clear halt on channel");
            }
        }
        Ok(())
    }

    fn clear_irq(&self) {
        self.INTERRUPT.set(0xFFFFFFFF);
        self.INTERRUPT_MASK.set(0);
    }

    fn wait_transmission(&self, timeout: Duration, timer: &'static SystemTimer) -> Result<LocalRegisterCopy<u32, CHANNEL_INTERRUPTS::Register>, &'static str> {
        timer.wait(Duration::from_millis(100));

        let tick = timer.tick_count();
        while self.INTERRUPT.matches_all(CHANNEL_INTERRUPTS::HALT::CLEAR) {
            if timer.tick_count() - tick > timeout {
                return Err("USB Channel Transmission Failed");
            }
        }
        Ok(self.INTERRUPT.extract())
    }

    pub fn handle_error(&self, irqs: &LocalRegisterCopy<u32, CHANNEL_INTERRUPTS::Register>) -> Result<HostChannelAction, HostChannelError> {
        if irqs.matches_any(&[CHANNEL_INTERRUPTS::AHB_ERROR::SET,CHANNEL_INTERRUPTS::DATA_TOGGLE_ERROR::SET]) {
            return Err(FatalError);
        }

        if irqs.matches_all(CHANNEL_INTERRUPTS::ACKNOWLEDGEMENT::SET) {
            return Ok(if irqs.matches_all(CHANNEL_INTERRUPTS::TRANSFERT_COMPLETE::SET) { Success } else { ResendSplit });
        }
        /* Check no transmission errors and if so deal with minor cases */
        if irqs.matches_all( CHANNEL_INTERRUPTS::STALL::CLEAR + CHANNEL_INTERRUPTS::BABBLE_ERROR::CLEAR + CHANNEL_INTERRUPTS::FRAME_OVERRUN::CLEAR ) {
            if irqs.matches_any( &[CHANNEL_INTERRUPTS::NEGATIVE_ACK::SET, CHANNEL_INTERRUPTS::NOT_YET_READY::SET]) {
                return Err(RecoverableError)
            } else {
                return Err(Timeout)
            }
        }

        Err(FatalError)

        /*

/*==========================================================================}
{				   INTERNAL HOST TRANSMISSION ROUTINES					    }
{==========================================================================*/

/*-INTERNAL: HCDCheckErrorAndAction -----------------------------------------
 Given a channel interrupt flags and whether packet was complete (not split)
 it will set sendControl structure with what to do next.
 24Feb17 LdB
 --------------------------------------------------------------------------*/
RESULT HCDCheckErrorAndAction(struct ChannelInterrupts interrupts, bool packetSplit, struct UsbSendControl* sendCtrl) {
	sendCtrl->ActionResendSplit = false;							// Make sure resend split flag is cleared
	sendCtrl->ActionRetry = false;									// Make sure retry flag is cleared
	/* First deal with all the fatal errors .. no use dealing with trivial errors if these are set */
	if (interrupts.AhbError) {										// Ahb error signalled .. which means packet size too large
		sendCtrl->ActionFatalError = true;							// This is a fatal error the packet size is all wrong
		return ErrorDevice;											// Return error device
	}
	if (interrupts.DataToggleError) {								// In bulk tranmission endpoint is supposed to toggle between data0/data1
		sendCtrl->ActionFatalError = true;							// Pretty sure this is a fatal error you can't fix it by resending
		return ErrorTransmission;									// Transmission error
	}
	/* Next deal with the fully successful case  ... we can return OK */
	if (interrupts.Acknowledgement) {								// Endpoint device acknowledge
		if (interrupts.TransferComplete) sendCtrl->Success = true;	// You can set the success flag
			else sendCtrl->ActionResendSplit = true;				// Action is to try sending split again
		sendCtrl->GlobalTries = 0;
		return OK;													// Return OK result
	}
	/* Everything else is minor error invoking a retry .. so first update counts */
	if (packetSplit) {
		sendCtrl->SplitTries++;										// Increment split tries as we have a split packet
		if (sendCtrl->SplitTries == 5) {							// Ridiculous number of split resends reached .. fatal error
			sendCtrl->ActionFatalError = true;						// This is a fatal error something is very wrong
			return ErrorTransmission;								// Transmission error
		}
		sendCtrl->ActionResendSplit = true;							// Action is to try sending split again
	} else {
		sendCtrl->PacketTries++;									// Increment packet tries as packet was not split
		if (sendCtrl->PacketTries == 3) {							// Ridiculous number of packet resends reached .. fatal error
			sendCtrl->ActionFatalError = true;						// This is a fatal error something is very wrong
			return ErrorTransmission;								// Transmission error
		}
		sendCtrl->ActionRetry = true;								// Action is to try sending the packet again
	}
	/* Check no transmission errors and if so deal with minor cases */
	if (!interrupts.Stall && !interrupts.BabbleError &&
		!interrupts.FrameOverrun) {									// No transmission error
		/* If endpoint NAK nothing wrong just demanding a retry */
		if (interrupts.NegativeAcknowledgement)						// Endpoint device NAK ..nothing wrong
			return ErrorTransmission;								// Simple tranmission error .. resend
		/* Next deal with device not ready case */
		if (interrupts.NotYet)
			return ErrorTransmission;								// Endpoint device not yet ... resend
		return ErrorTimeout;										// Let guess program just timed out
	}
	/* Everything else updates global count as it is serious */
	sendCtrl->GlobalTries++;										// Increment global tries
																	/* If global tries reaches 3 .. its a fatal error */
	if (sendCtrl->GlobalTries == 3) {								// Global tries has reached 3
		sendCtrl->ActionRetry = false;								// Clear retry action flag .. it's fatal
		sendCtrl->ActionResendSplit = false;						// Clear retyr sending split again .. it's fatal
		sendCtrl->ActionFatalError = true;							// This is a fatal error to many global errors
		return ErrorTransmission;									// Transmission error
	}
	/* Deal with stall */
	if (interrupts.Stall) {											// Stall signalled .. device endpoint problem
		return ErrorStall;											// Return the stall error
	}
	/* Deal with true transmission errors */
	if ((interrupts.BabbleError) ||									// Bable error is a packet transmission problem
		(interrupts.FrameOverrun) ||								// Frame overrun error means stop bit failed at packet end
		(interrupts.TransactionError))
	{
		return ErrorTransmission;									// Transmission error
	}
	return ErrorGeneral;											// If we get here then no idea why error occured (probably program error)
}
         */
    }
/*
    pub fn send_receive_buffer(&self, timer: &'static SystemTimer, pipe: &Pipe, buffer: &[u8], packet_id: u32) -> Result<(), &'static str> {

        debugln!("INTERRUPT register {:b}", self.INTERRUPT.get());
        /* Clear all existing interrupts. */
        self.clear_irq();

        debugln!("INTERRUPT register {:b}", self.INTERRUPT.get());

        let max_packet_size = pipe.size();
        let low_speed = pipe.o.matches_all(PIPE::SPEED::Low);

        /* Program the channel. */
        self.CHARACTERISTIC.write(HOST_CHANNEL_CHARACTERISTIC::DEVICE_ADDRESS.val(pipe.o.read(PIPE::NUMBER))
            + HOST_CHANNEL_CHARACTERISTIC::ENDPOINT_NUMBER.val(pipe.o.read(PIPE::ENDPOINT))
            + HOST_CHANNEL_CHARACTERISTIC::ENDPOINT_DIRECTION.val(pipe.c.read((PIPE_CONTROL::DIRECTION)))
            + if low_speed { HOST_CHANNEL_CHARACTERISTIC::LOW_SPEED::SET } else { HOST_CHANNEL_CHARACTERISTIC::LOW_SPEED::CLEAR }
            + HOST_CHANNEL_CHARACTERISTIC::ENDPOINT_TYPE.val(pipe.c.read(PIPE_CONTROL::TRANSFER_TYPE))
            + HOST_CHANNEL_CHARACTERISTIC::MAX_PACKET_SIZE.val(max_packet_size)
            + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_ENABLED::CLEAR
            + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_DISABLED::CLEAR);

        /* Clear and setup split control to low speed devices */
        if !pipe.o.matches_all(PIPE::SPEED::High) {
            let low_speed_point = pipe.o.read(PIPE::LOW_SPEED_NODE_POINT);
            let low_speed_port = pipe.o.read(PIPE::LOW_SPEED_NODE_PORT);
            debugln!("Setting split control, addr: {} port: {}, packetSize: PacketSize: {}",
                       low_speed_point, low_speed_port, max_packet_size);
            self.SPLIT_CONTROL.write(HOST_SPLIT_CONTROL::SPLIT_ENABLE::SET
                + HOST_SPLIT_CONTROL::HUB_ADDRESS.val(low_speed_point)
                + HOST_SPLIT_CONTROL::PORT_ADDRESS.val(low_speed_port));
        }

        /* Set transfer size. */
        let buffer_length = buffer.len() as u32;
        let packet_count = if pipe.o.matches_all(PIPE::SPEED::Low)
        { (buffer_length + 7) / 8 } else { (buffer_length + max_packet_size - 1) / max_packet_size };

        self.TRANSFER_SIZE.write(CHANNEL_TRANSFER_SIZE::SIZE.val(buffer_length)
            + CHANNEL_TRANSFER_SIZE::PACKET_COUNT.val(if packet_count > 0 { packet_count } else { 1 })
            + CHANNEL_TRANSFER_SIZE::PACKET_ID.val(packet_id));

        loop {
            // Clear any left over channel interrupts
            self.clear_irq();
            // Clear any left over split
            self.SPLIT_CONTROL.write(HOST_SPLIT_CONTROL::COMPLETE_SPLIT::CLEAR);

            // set buffer addr (4 byte aligned and with physical addr rather than ARM addr)
            // TODO : create macro rather than oring the buffer
            self.DMA_ADDR.set(buffer.as_ptr() as u32 | 0xC0000000);

            debugln!("INTERRUPT register {:b}", self.INTERRUPT.get());

            self.CHARACTERISTIC.write(HOST_CHANNEL_CHARACTERISTIC::PACKETS_PER_FRAME.val(1)
                + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_ENABLED::SET
                + HOST_CHANNEL_CHARACTERISTIC::CHANNEL_DISABLED::CLEAR);

            let irq_register = self.wait_transmission(5000, timer)?;

            debugln!(" IRQ register after sending a message {:b} {:x} = {:?}",  irq_register.get(), irq_register.get(), self.handle_error(&irq_register));
            debugln!("Transfert Size after write {}", self.TRANSFER_SIZE.read(CHANNEL_TRANSFER_SIZE::SIZE));
            debugln!("characteristic : {:x}", self.CHARACTERISTIC.get());

            if self.TRANSFER_SIZE.read(CHANNEL_TRANSFER_SIZE::PACKET_COUNT) == 0 {
                break;
            }
            break;
        }
        /*
        sendCtrl.PacketTries = 0;										// Zero packet tries
	do {



		// Polling wait on transmission only option right now .. other options soon :-)
		if (HCDWaitOnTransmissionResult(5000, pipectrl.Channel, &tempInt) != OK) {
			LOG("HCD: Request on channel %i has timed out.\n", pipectrl.Channel);// Log the error
			return ErrorTimeout;									// Return timeout error
		}

		tempSplit = DWC_HOST_CHANNEL[pipectrl.Channel].SplitCtrl;	// Fetch the split details
		result = HCDCheckErrorAndAction(tempInt,
			tempSplit.split_enable, &sendCtrl);						// Check transmisson RESULT and set action flags
		if (result) LOG("Result: %i Action: 0x%08x tempInt: 0x%08x tempSplit: 0x%08x Bytes sent: %i\n",
			result, (unsigned int)sendCtrl.Raw32, (unsigned int)tempInt.Raw32,
			(unsigned int)tempSplit.Raw32, result ? 0 : DWC_HOST_CHANNEL[pipectrl.Channel].TransferSize.size);
		if (sendCtrl.ActionFatalError) return result;				// Fatal error occured we need to bail

		sendCtrl.SplitTries = 0;									// Zero split tries count
		while (sendCtrl.ActionResendSplit) {						// Decision was made to resend split
			/* Clear channel interrupts */
			DWC_HOST_CHANNEL[pipectrl.Channel].Interrupt.Raw32 = 0xFFFFFFFF;
			DWC_HOST_CHANNEL[pipectrl.Channel].InterruptMask.Raw32 = 0x0;

			/* Set we are completing the split */
			tempSplit = DWC_HOST_CHANNEL[pipectrl.Channel].SplitCtrl;
			tempSplit.complete_split = true;						// Set complete split flag
			DWC_HOST_CHANNEL[pipectrl.Channel].SplitCtrl = tempSplit;

			/* Launch transmission */
			tempChar = DWC_HOST_CHANNEL[pipectrl.Channel].Characteristic;
			tempChar.channel_enable = true;
			tempChar.channel_disable = false;
			DWC_HOST_CHANNEL[pipectrl.Channel].Characteristic = tempChar;

			// Polling wait on transmission only option right now .. other options soon :-)
			if (HCDWaitOnTransmissionResult(5000, pipectrl.Channel, &tempInt) != OK) {
				LOG("HCD: Request split completion on channel:%i has timed out.\n", pipectrl.Channel);// Log error
				return ErrorTimeout;								// Return timeout error
			}

			tempSplit = DWC_HOST_CHANNEL[pipectrl.Channel].SplitCtrl;// Fetch the split details again
			result = HCDCheckErrorAndAction(tempInt,
				tempSplit.split_enable, &sendCtrl);					// Check RESULT of split resend and set action flags
			//if (result) LOG("Result: %i Action: 0x%08lx tempInt: 0x%08lx tempSplit: 0x%08lx Bytes sent: %i\n",
			//	result, sendCtrl.RawUsbSendContol, tempInt.RawInterrupt, tempSplit.RawSplitControl, RESULT ? 0 : DWC_HOST_CHANNEL[pipectrl.Channel].TransferSize.TransferSize);
			if (sendCtrl.ActionFatalError) return result;			// Fatal error occured bail
			if (sendCtrl.LongerDelay) timer_wait(10000);			// Not yet response slower delay
				else timer_wait(2500);								// Small delay between split resends
		}

		if (sendCtrl.Success) {										// Send successful adjust buffer position
			unsigned int this_transfer;
			this_transfer = DWC_HOST_CHANNEL[pipectrl.Channel].TransferSize.size;

			if (((uint32_t)(intptr_t)&buffer[offset] & 3) != 0) {	// Buffer address is unaligned

				// Since our buffer is unaligned for IN endpoints
				// Copy the data from the the aligned buffer to the buffer
				// We know the aligned buffer was used because it is unaligned
				if (pipectrl.Direction == USB_DIRECTION_IN)
				{
					memcpy(&buffer[offset], aligned_bufs[pipectrl.Channel], this_transfer);
				}
			}

			offset = bufferLength - this_transfer;
		}

	} while (DWC_HOST_CHANNEL[pipectrl.Channel].TransferSize.packet_count > 0);// Full data not sent
         */

        Ok(())
    }*/
}