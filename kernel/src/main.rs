#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_a::regs::*;

mod kernel;

/// Reset function.
///
/// Initializes the bss section before calling into the user's `main()`.
/// No need to park other processor, it has been done by boot.c
#[link_section = ".text.boot"]
#[no_mangle]
unsafe fn reset() -> ! {
    const STACK_START: u64 = 0x3F_000_000;

    SP.set(STACK_START);

    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    kernel::main();
}
