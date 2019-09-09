#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]

#[macro_use] extern crate mmio;
use cortex_a::asm;
use cortex_a::regs::*;
use memory::descriptors::{KERNEL_VIRTUAL_LAYOUT, PROGRAM_VIRTUAL_LAYOUT};

mod memory;
mod exceptions;

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    debugln!("{:?}", info);
    asm::wfe();
    loop {}
}

/// Entrypoint of the kernel.
///
/// No need to park other processor it has been done by boot.c
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _upper_kernel() -> ! {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let gpio = mmio::GPIO::new(memory::map::virt::GPIO_BASE);
    let mut v_mbox = mmio::Mbox::new(memory::map::virt::MBOX_BASE);
    let uart = mmio::Uart::new(memory::map::virt::UART_BASE);

    mmio::LOGGER.appender(uart.into());
    debugln!("UART live in upper level");

    exceptions::init();
    shared::memory::mmu::setup_kernel_tables(&KERNEL_VIRTUAL_LAYOUT, memory::map::physical::KERN_MMU_START);
    shared::memory::mmu::setup_user_tables(&PROGRAM_VIRTUAL_LAYOUT, memory::map::physical::USER_MMU_START);
    debugln!("MMU re-configured");

    let bytes = include_bytes!("../../program.img");
    debugln!("copying program from {:p} to {:#x} with len {}", &bytes[0] as *const u8, memory::map::physical::PROG_START, bytes.len());
    core::ptr::copy(&bytes[0] as *const u8, memory::map::physical::PROG_START as *mut u8, bytes.len());

    debugln!("JUMP to program");

    SP_EL0.set(0x00400000);
    ELR_EL1.set(memory::map::physical::PROG_START as u64);
    asm::eret();

    loop {}
}