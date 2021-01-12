use mmio::Uart;
use mmio::io::{IoResult, Reader, Writer};
use crate::memory::descriptors::KERNEL_VIRTUAL_LAYOUT;
use cortex_a::regs::{SP, RegisterReadWrite, ELR_EL2, SP_EL1, HCR_EL2, CNTHCTL_EL2, CNTVOFF_EL2, SPSR_EL2};
use cortex_a::asm;
use crate::{STACK_START, memory, exceptions};
use mmio::logger::Logger;

#[inline]
pub fn setup_el1_and_jump_high() -> ! {

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

    // Disable MMU to reset previous MMU configuration into the new kernel
    shared::memory::mmu::disable();

    // Second, let the link register point to reset().
    ELR_EL2.set(reset as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once
    // we "return" to it.
    SP_EL1.set(STACK_START);

    // Use `eret` to "return" to EL1. This will result in execution of
    // `reset()` in EL1.
    asm::eret()
}

unsafe fn reset() -> ! {

    let mut uart = mmio::Uart::new(memory::map::physical::UART_BASE);

    match setup_mmu() {
        Err(err) => panic!("setup mmu failed : {}", err),
        _ => {}
    }
    match load_kernel(&mut uart) {
        Err(_err) => panic!("loading kernel failed"),
        _ => {}
    }

    debugln!("jump to upper level");
    let upper_main: extern "C" fn() -> ! = core::mem::transmute(memory::map::virt::KERN_START);
    SP.set(memory::map::virt::KERN_STACK_START as u64);
    upper_main()
}

fn setup_mmu() -> Result<(), &'static str>{
    shared::memory::mmu::setup_user_tables(&KERNEL_VIRTUAL_LAYOUT)?;
    shared::memory::mmu::setup_kernel_tables(&KERNEL_VIRTUAL_LAYOUT)?;
    shared::memory::mmu::init()
}

unsafe fn load_kernel(uart: &mut Uart) -> IoResult<()> {
    debugln!("load kernel");
    uart.clear()?;
    uart.writes("\x03\x03\x03")?;
    let len = uart.read_dword()?;
    uart.writes("\x03\x03\x03")?;
    uart.write_dword(len);
    uart.writes("\x03\x03\x03")?;
    let kernel_addr: *mut u8 = memory::map::physical::KERN_START as *mut u8;
    unsafe {
        // Read the kernel byte by byte.
        for i in 0..len {
            core::ptr::write_volatile(kernel_addr.offset(i as isize), uart.read_char()?);
        }
    }
    uart.writes("\x03\x03\x03")?;
    Ok(())
}