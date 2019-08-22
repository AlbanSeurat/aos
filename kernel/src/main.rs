#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]

use cortex_a::asm;

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    asm::wfe();
    loop {}
}

use cortex_a::regs::*;
mod kernel;

/// Reset function.
///
/// Initializes the bss section before calling into the user's `main()`.
/// No need to park other processor, it has been done by boot.c
unsafe fn reset() -> ! {

    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    match unsafe { kernel::memory::mmu::init() } {
        Err(s) => {
            debugln!("MMU error: {}\n", s);
        }
        Ok(()) => ()
    }

    SP.set(0xFFFFFFFFC0000000 + SP.get());
    let upper_main: extern "C" fn() = unsafe { core::mem::transmute(0xFFFFFFFFC0000000 + kernel::main as usize) };
    upper_main();

    loop {}
}

/// Prepare and execute transition from EL2 to EL1.
#[inline]
fn setup_and_enter_el1_from_el2() -> ! {

    const STACK_START: u64 = 0x3B_000_000; // upper end of the kernel segment (2Mb max for a long time)

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
    ELR_EL2.set(reset as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once
    // we "return" to it.
    SP_EL1.set(STACK_START);

    // Use `eret` to "return" to EL1. This will result in execution of
    // `reset()` in EL1.
    asm::eret()
}

/// Entrypoint of the processor.
///
/// No need to park other processor it has been done by boot.c
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _boot_cores() -> ! {

    const EL2: u32 = CurrentEL::EL::EL2.value;

    if EL2 == CurrentEL.get() {
        setup_and_enter_el1_from_el2()
    }

    // if we are already in EL1 call directly reset
    reset();
}