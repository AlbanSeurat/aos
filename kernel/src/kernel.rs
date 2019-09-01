use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{TTBR0_EL1, ELR_EL1, RegisterReadWrite};
use cortex_a::asm;
use core::fmt::Write;
use crate::reset;
use crate::kernel::memory::KERNEL_VIRTUAL_LAYOUT;
use crate::kernel::memory::kernel_mem_range::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use crate::kernel::memory::map::physical::KERN_START;
use crate::kernel::exceptions::set_vbar_el1_checked;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod memory;
mod exceptions;
mod devices;

static mut UART : Uart = devices::hw::Uart::new(memory::map::virt::UART_BASE);

pub fn main() -> ! {

    extern "C" {
        static __exception_vectors_start: u64;
    }

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

    unsafe {
        let exception_vectors_start: u64 = &__exception_vectors_start as *const _ as u64;
        set_vbar_el1_checked(exception_vectors_start)
    }

    debugln!("main has been called in upper space {:x?}", pc);

    ELR_EL1.set(0);
    asm::eret();

    loop {}
}
