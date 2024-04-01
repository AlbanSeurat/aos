#![no_std]
#![no_main]
#![feature(core_intrinsics)]

#[macro_use]
extern crate mmio;

use core::arch::asm;
use aarch64_cpu::asm;
use aarch64_cpu::registers::*;
use qemu_exit::QEMUExit;
use CurrentEL::EL::EL2;

mod exceptions;
mod memory;
mod stage1;

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

const STACK_START: u64 = 0x80_000;

#[no_mangle]
extern "C" fn test() -> u64 {
    let a = 544554;
    if a < STACK_START {
        return 33234;
    }
    return 544554;
}

unsafe fn start() -> ! {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let gpio = mmio::GPIO::new(memory::map::physical::GPIO_BASE);
    let mut v_mbox = mmio::Mbox::new(memory::map::physical::MBOX_BASE);
    let uart = mmio::Uart::new(memory::map::physical::UART_BASE);

    match uart.init(&mut v_mbox, &gpio) {
        Ok(_) => {
            mmio::LOGGER.appender(uart.into());
        }
        Err(_) => loop {
            panic!("uart not properly setup");
        },
    }
    if EL2.value != CurrentEL.get() {
        panic!("Kernel not started in Exception Level 2");
    }
    exceptions::init_el2();

    // call hypervisor to reset the kernel load
    asm!("HVC 1");

    // should never be called
    loop {
        asm::wfi();
    }
}

/// Entrypoint of the processor.
///
/// No need to park other processor it has been done by boot.c
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _boot_cores() -> ! {
    const CORE_0: u64 = 0;
    const CORE_MASK: u64 = 0x3;

    if CORE_0 == MPIDR_EL1.get() & CORE_MASK {
        SP.set(STACK_START);
        start();
    }
    loop {
        asm::wfe();
    }
}