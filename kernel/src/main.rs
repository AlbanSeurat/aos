#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(duration_constants)]
#![feature(llvm_asm)]

#[macro_use] extern crate mmio;
use memory::descriptors::{KERNEL_VIRTUAL_LAYOUT, PROGRAM_VIRTUAL_LAYOUT};
use qemu_exit::QEMUExit;
use cortex_a::regs::{SP_EL0, ELR_EL1, RegisterReadWrite};
use cortex_a::asm;
use crate::exceptions::BCMDEVICES;
use shared::memory::mmu::VIRTUAL_ADDR_START;

mod memory;
mod exceptions;
mod scheduler;

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info);
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure()
}

/// Entrypoint of the kernel.
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _upper_kernel() -> ! {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let _gpio = mmio::GPIO::new(memory::map::virt::GPIO_BASE);
    let v_mbox = mmio::Mbox::new(memory::map::virt::MBOX_BASE);
    let uart = mmio::Uart::new(memory::map::virt::UART_BASE);
    let _dwhci = mmio::DWHCI::new(memory::map::virt::USB_BASE);
    let _irq = mmio::IRQ::new(memory::map::virt::IRQ_BASE);
    let mut console = mmio::FrameBufferConsole::new(v_mbox, VIRTUAL_ADDR_START);
    mmio::LOGGER.appender(uart.into());
    mmio::SCREEN.appender( console.into());
    println!("MMIO live in upper level");

    exceptions::init();
    println!("Exception Handling initialized");

    match setup_mmu() {
        Err(err) => panic!("setup mmu failed : {}", err),
        _ => {}
    }

    mmio::timer::LocalTimer::setup(&BCMDEVICES);

    let bytes = include_bytes!("../../program.img");
    println!("copying program from {:p} to {:#x} with len {}", bytes as *const u8, memory::map::physical::PROG_START, bytes.len());
    core::ptr::copy(bytes as *const u8, memory::map::physical::PROG_START as *mut u8, bytes.len());

    println!("JUMP to program");

    SP_EL0.set(0x00400000);
    ELR_EL1.set(memory::map::physical::PROG_START as u64);
    asm::eret();
}

fn setup_mmu() -> Result<(), &'static str> {
    shared::memory::mmu::setup_kernel_tables(&KERNEL_VIRTUAL_LAYOUT)?;
    shared::memory::mmu::setup_user_tables(&PROGRAM_VIRTUAL_LAYOUT)?;
    unsafe { print!("MMU Kernel mapping : \n{}", shared::memory::mmu::kernel_tables()); }
    unsafe { print!("MMU Program mapping : \n{}", shared::memory::mmu::user_tables()); }
    println!("MMU re-configured");
    Ok(())
}