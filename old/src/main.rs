#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]

use cortex_a::asm;
use cortex_a::regs::*;
mod kernel;

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    asm::wfe();
    loop {}
}

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;

    static mut __kernel_start: u64;
    static mut __kernel_end: u64;
}
const STACK_START: u64 = 0x3B_000_000; // upper end of the kernel segment (2Mb max for a long time)
const KERN_START: u64 = 0x3A_E00_000; // lower end of the kernel segment

const ARM_STARTUP: u64 = 0x80_000;

unsafe fn reset() -> ! {

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    match { kernel::memory::mmu::init() } {
        Err(_) => (),
        Ok(()) => ()
    }

    SP.set(kernel::memory::map::virt::START as u64 + SP.get());
    let upper_main: extern "C" fn() = { core::mem::transmute(kernel::memory::map::virt::START + kernel::main as usize) };
    upper_main();

    loop {}
}

#[inline]
fn setup_el1_and_jump_high() -> ! {

    // Enable timer counter registers for EL1
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exceptions return.
    //
    // First, fake a saved program status, where all interrupts were
    // masked and SP_EL1 was used as a stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to reset().
    ELR_EL2.set(KERN_START - ARM_STARTUP + reset as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once
    // we "return" to it.
    SP_EL1.set(STACK_START);

    // Use `eret` to "return" to EL1. This will result in execution of
    // `reset()` in EL1.
    asm::eret()
}

#[inline]
fn move_kernel() {
    unsafe {
        let kernel_size = 0x00200000; // 2Mb (max)
        asm!("1: \
        ldr X3, [X1], #8; \
        str X3, [X0], #8; \
        subs X2, X2, #8; \
        bge 1b;" :: "{X1}"(ARM_STARTUP), "{X0}"(KERN_START), "{X2}"(kernel_size) : "X3" : "volatile");
    }
}

/// Entrypoint of the processor.
///
/// No need to park other processor it has been done by boot.c
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _boot_cores() -> ! {

    const EL2: u32 = CurrentEL::EL::EL2.value;
    const CORE_0: u64 = 0;
    const CORE_MASK: u64 = 0x3;

    if CORE_0 == MPIDR_EL1.get() & CORE_MASK {
        move_kernel();
        if EL2 == CurrentEL.get() {
            setup_el1_and_jump_high()
        }
    }
    // if not core0 or not EL2, infinitely wait for events
    loop {
        asm::wfe();
    }
}