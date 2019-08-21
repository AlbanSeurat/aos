use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{LR, TTBR0_EL1, RegisterReadWrite};
use cortex_a::asm;
use core::fmt::Write;
use crate::reset;
use crate::kernel::memory::KERNEL_VIRTUAL_LAYOUT;
use crate::kernel::memory::kernel_mem_range::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use crate::kernel::memory::map::physical::KERN_START;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod memory;
mod devices;

static mut UART : Uart = devices::hw::Uart::new(memory::map::virt::UART_BASE);

pub fn main() -> ! {

    // this will make break every attempt to get data outside kernel address space
    TTBR0_EL1.set(0);

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::virt::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    let mut console : Console = Console::new(lfb);
    log!(console, "Starting AoS...");

    let mut pc : u64;
    unsafe {
        asm!("adr $0, ." : "=r"(pc));
    }

    debugln!("main has been called in upper space {:x?}", pc);

    loop {}
}
