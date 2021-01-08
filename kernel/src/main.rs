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
use mmio::{BCMDeviceMemory, Uart, DMA};
use crate::global::{UART, BCMDEVICES};
use crate::process::k_create_process;

mod memory;
mod exceptions;
mod global;
mod process;

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
    match setup_mmu() {
        Err(err) => panic!("setup mmu failed : {}", err),
        _ => {}
    }
    DMA.set(mmio::MemoryRegion::new(memory::map::physical::MMA_MEMORY_START, memory::map::physical::MMA_MEMORY_END));
    let v_mbox = mmio::Mbox::new_with_dma(memory::map::virt::MBOX_BASE);
    mmio::LOGGER.appender(UART.into());
    mmio::SCREEN.appender(UART.into());
    exceptions::init();
    let mut console = mmio::FrameBufferConsole::new(v_mbox, VIRTUAL_ADDR_START);
    mmio::SCREEN.appender( console.into());

    unsafe { print!("MMU Kernel mapping : \n{}", shared::memory::mmu::kernel_tables()); }
    unsafe { print!("MMU Program mapping : \n{}", shared::memory::mmu::user_tables()); }

    // setup IRQs
    //UART.enable_rx_irq(&irq, &bcm);
    mmio::timer::LocalTimer::setup(&BCMDEVICES);

    k_create_process()
}

fn setup_mmu() -> Result<(), &'static str> {
    shared::memory::mmu::setup_kernel_tables(&KERNEL_VIRTUAL_LAYOUT)?;
    shared::memory::mmu::setup_user_tables(&PROGRAM_VIRTUAL_LAYOUT)?;
    Ok(())
}