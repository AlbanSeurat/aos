#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(duration_constants)]
#![feature(llvm_asm)]

#[macro_use] extern crate mmio;
use memory::descriptors::{KERNEL_VIRTUAL_LAYOUT, PROGRAM_VIRTUAL_LAYOUT};
use cortex_a::regs::{SP_EL0, ELR_EL1, SPSR_EL1, RegisterReadWrite};
use cortex_a::asm;
use shared::memory::mmu::VIRTUAL_ADDR_START;
use mmio::{BCMDeviceMemory, Uart};
use crate::global::{UART, BCMDEVICES};

mod memory;
mod exceptions;
mod scheduler;
mod global;

extern "C" {
    // Boundaries of the .bss section, provided by the linker script
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

/// Entrypoint of the kernel.
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _upper_kernel() -> ! {

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    let v_mbox = mmio::Mbox::new(memory::map::virt::MBOX_BASE);
    let _dwhci = mmio::DWHCI::new(memory::map::virt::USB_BASE);
    let irq = mmio::IRQ::new(memory::map::virt::IRQ_BASE);
    let bcm = mmio::BCMDeviceMemory::new(memory::map::peripheral::START);

    mmio::LOGGER.appender(UART.into());
    mmio::SCREEN.appender( UART.into());

    //let mut console = mmio::FrameBufferConsole::new(v_mbox, VIRTUAL_ADDR_START);

    println!("MMIO live in upper level");

    exceptions::init();
    println!("Exception Handling initialized");

    match setup_mmu() {
        Err(err) => panic!("setup mmu failed : {}", err),
        _ => {}
    }

    // setup IRQs
    //UART.enable_rx_irq(&irq, &bcm);
    //mmio::timer::LocalTimer::setup(&BCMDEVICES);

    let bytes = include_bytes!("../../program.img");
    println!("copying program from {:p} to {:#x} with len {}", bytes as *const u8, memory::map::physical::PROG_START, bytes.len());
    core::ptr::copy(bytes as *const u8, memory::map::physical::PROG_START as *mut u8, bytes.len());

    println!("JUMP to program at {:x}", memory::map::physical::PROG_START as u64);
    SP_EL0.set(0x00400000);
    // Indicate that we "return" to EL0
    SPSR_EL1.write(SPSR_EL1::M::EL0t);
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