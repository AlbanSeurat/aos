mod interruptions;
mod syscalls;

pub use interruptions::{disable_irq, enable_irq};

use shared::exceptions::set_vbar_el1_checked;
use shared::exceptions::handlers::ExceptionContext;
use register::cpu::RegisterReadWrite;
use register::cpu::RegisterReadOnly;
use cortex_a::regs::{ESR_EL1, FAR_EL1, SPSR_EL1};
use cortex_a::{barrier, asm};

extern "C" {
    static __exception_vectors_start: u64;
}

pub unsafe fn init() {
    let exception_vectors_start: u64 = &__exception_vectors_start as *const _ as u64;
    set_vbar_el1_checked(exception_vectors_start);
    barrier::isb(barrier::SY);
}

/// The default exceptions, invoked for every exceptions type unless the handler
/// is overwritten.
#[no_mangle]
unsafe extern "C" fn default_exception_handler(e: &ExceptionContext) {
    debugln!("Unknown Exception Context");
    debug_halt(e);
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(e: &ExceptionContext) {
    disable_irq();
    debugln!("Current IRQ handling");
    enable_irq();
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_synchronous(e : &ExceptionContext) {

    if ESR_EL1.read(ESR_EL1::EC) == 0x15 { // SVC call
        match ESR_EL1.read(ESR_EL1::ISS) {
            1 => syscalls::syscall_one(e.gpr.x[0] as *const u8, e.gpr.x[1] as usize),
            2 => syscalls::syscall_two(e.gpr.x[0] as u64),
            _ => ()
        }
    } else {
        debugln!("Synchronous exception lower EL");
        debug_halt(e);
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &ExceptionContext) {
    debugln!("Synchronous exception current EL");
    debug_halt(e);
}

fn debug_halt(e: &ExceptionContext) {
    debugln!("GPR : {:x?}", e.gpr);
    debugln!("ESR : {:#x?}/{:#x?}", ESR_EL1.read(ESR_EL1::EC), ESR_EL1.get());
    debugln!("FAR : {:#x?}", FAR_EL1.get());
    debugln!("ELR : {:#x?}", e.elr_el1);
    debugln!("PSTATE: {:#x?}", SPSR_EL1.get());

    loop {
        cortex_a::asm::wfe()
    }
}
