use crate::kernel::devices::virt::Console;
use crate::kernel::devices::hw::{Uart, FrameBuffer};
use cortex_a::regs::{TTBR0_EL1, ELR_EL1, RegisterReadWrite, SP_EL0, SCTLR_EL1};
use cortex_a::asm;
use core::fmt::Write;
use crate::reset;
use crate::kernel::memory::KERNEL_VIRTUAL_LAYOUT;
use crate::kernel::memory::kernel_mem_range::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use crate::kernel::memory::map::physical::KERN_START;
use crate::kernel::exceptions::set_vbar_el1_checked;
use crate::kernel::memory::mmu::{map_user_table};
use core::intrinsics::copy;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod memory;
mod exceptions;
mod devices;

static mut UART : Uart = devices::hw::Uart::new(memory::map::virt::UART_BASE);

pub static ProgramDescriptor : Descriptor =  Descriptor {
    virtual_range: || {
        RangeInclusive::new(0, 0x00200000 - 1)
    },
    map: Some(Mapping {
        translation: Translation::Identity,
        attribute_fields: AttributeFields {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadWriteUser,
            execute_never: false,
        },
    }),
};

pub fn main() -> ! {

    extern "C" {
        static __exception_vectors_start: u64;
    }

    unsafe {
        memory::mmu::reset_el0();
    }

    let mut v_mbox = devices::hw::VideocoreMbox::new(memory::map::virt::VIDEOCORE_MBOX_BASE);
    let lfb : FrameBuffer = FrameBuffer::new(&mut v_mbox);
    let mut console : Console = Console::new(lfb);
    log!(console, "Starting AoS...");

    unsafe {
        let exception_vectors_start: u64 = &__exception_vectors_start as *const _ as u64;
        set_vbar_el1_checked(exception_vectors_start)
    }

    unsafe {
        map_user_table(&ProgramDescriptor);
    }

    SP_EL0.set(0x00200000 - 1);
    ELR_EL1.set(0);
    asm::eret();

    loop {}
}
