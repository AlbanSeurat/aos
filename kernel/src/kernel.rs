use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{SP, RegisterReadWrite};
use core::fmt::Write;
use crate::reset;
use crate::kernel::memory::KERNEL_VIRTUAL_LAYOUT;
use crate::kernel::memory::kernel_mem_range::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use crate::kernel::memory::map::virt::KERN_START;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod memory;
mod devices;

static mut UART : Uart = devices::hw::Uart::new(memory::map::physical::UART_BASE);

static test : Descriptor =
    Descriptor {
        virtual_range: || {
            RangeInclusive::new(
                0x3ae0b000 as usize,
                0x3ae0c000 - 1 as usize
            )
        },
        map: Some(Mapping {
            translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            }
        })
    };

pub fn main() -> ! {

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::physical::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    let mut console : Console = Console::new(lfb);
    log!(console, "Starting AoS...");

    // update MMU tables and try to write 10 into newly descriptor described in the table
    unsafe {
        memory::mmu::new(&test, 0x3ae0b000 - KERN_START);
        core::ptr::write_volatile(0x3ae0b000 as *mut u64, 10 as u64);
    }

    loop {}
}

