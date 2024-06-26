#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(duration_constants)]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate mmio;

use aarch64_cpu::asm;

use memory::descriptors::{KERNEL_VIRTUAL_LAYOUT, PROGRAM_VIRTUAL_LAYOUT};
use mmio::{DMA, HEAP, IRQ};
use shared::memory::mmu::{VIRTUAL_ADDR_START};

use crate::global::{BCMDEVICES, UART, TIMER};
use crate::scheduler::process::{create_init_program, create_tmp_init_program};

mod memory;
mod exceptions;
mod global;
mod scheduler;

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
    exceptions::init();
    unsafe {
        DMA.lock().init(memory::map::physical::MMA_MEMORY_START,
                        memory::map::physical::MMA_MEMORY_END - memory::map::physical::MMA_MEMORY_START);
    }
    let v_mbox = mmio::Mbox::new_with_dma(memory::map::virt::MBOX_BASE);
    mmio::LOGGER.appender(UART.into());
    let console = mmio::FrameBufferConsole::new(v_mbox, VIRTUAL_ADDR_START);
    mmio::SCREEN.appender( console.into());

    unsafe { print!("MMU Kernel mapping : \n{}", shared::memory::mmu::kernel_tables()); }
    unsafe { print!("MMU Program mapping : \n{}", shared::memory::mmu::user_tables()); }

    unsafe {
        HEAP.lock().init(memory::map::virt::KERNEL_HEAP_START,
                         memory::map::virt::KERNEL_HEAP_END - memory::map::virt::KERNEL_HEAP_START);
    }

    // setup IRQs
    //UART.enable_rx_irq(&irq, &bcm);

    create_tmp_init_program();
    create_tmp_init_program();
    create_tmp_init_program();
    create_tmp_init_program();
    create_tmp_init_program();

    create_init_program();

    TIMER.setup(&BCMDEVICES);
    unsafe { IRQ::enable(); }

    loop {
        asm::wfi();
    }
}

fn setup_mmu() -> Result<(), &'static str> {
    shared::memory::mmu::setup_kernel_tables(&KERNEL_VIRTUAL_LAYOUT)?;
    shared::memory::mmu::setup_user_tables(&PROGRAM_VIRTUAL_LAYOUT)?;
    Ok(())
}
