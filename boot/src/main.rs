#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]

#[macro_use] extern crate mmio;

use cortex_a::asm;
use cortex_a::regs::*;

mod exceptions;

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
}

const STACK_START: u64 = 0x80_000;
const MMIO_BASE: usize = 0x3F00_0000;
const GPIO_BASE: usize = MMIO_BASE + 0x0020_0000;
const MBOX_BASE: usize = MMIO_BASE + 0x0000_B880;
const UART_BASE: usize = MMIO_BASE + 0x0020_1000;

unsafe fn reset() -> ! {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let gpio = mmio::GPIO::new(GPIO_BASE);
    let mut v_mbox = mmio::Mbox::new(MBOX_BASE);
    let uart = mmio::Uart::new(UART_BASE);

    match uart.init(&mut v_mbox, &gpio) {
        Ok(_) => {
            mmio::LOGGER.appender(uart.into());
            debugln!("UART is live!");
        }
        Err(_) => loop {
            cortex_a::asm::wfe() // If UART fails, abort early
        },
    }
    exceptions::init();

    debugln!("write to 0x80000000000");
    core::ptr::write_volatile(0x80000000000 as *mut u64, 0);

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
    const CORE_0: u64 = 0;
    const CORE_MASK: u64 = 0x3;

    if CORE_0 == MPIDR_EL1.get() & CORE_MASK {

        if EL2 == CurrentEL.get() {
            setup_el1_and_jump_high();
        }
        reset();
    }
    // if not core0 or not EL2, infinitely wait for events
    loop {
        asm::wfe();
    }
}