
mod syscalls;
mod interruptions;

use shared::exceptions::handlers::ExceptionContext;
use aarch64_cpu::registers::{ESR_EL1, FAR_EL1, SPSR_EL1, SP_EL0, CurrentEL, Readable, Writeable};
use qemu_exit::QEMUExit;
use aarch64_cpu::asm::*;
use crate::exceptions::interruptions::irq_handler;

extern "C" {
    static __exception_vectors_start: u64;
}

pub unsafe fn init() {
    let exception_vectors_start: u64 = &__exception_vectors_start as *const _ as u64;
    aarch64_cpu::registers::VBAR_EL1.set(exception_vectors_start);
    barrier::isb(barrier::SY);
}


/// The default exceptions, invoked for every exceptions type unless the handler
/// is overwritten.
#[no_mangle]
unsafe extern "C" fn default_exception_handler(e: &ExceptionContext)  {
    debug_halt("default_exception_handler", e);
}

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &ExceptionContext) {
    if ESR_EL1.read(ESR_EL1::EC) == 0x15 { // SVC call
        syscalls::syscalls(e)
    } else {
        debug_halt("current_elx_synchronous", e);
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_synchronous(e : &ExceptionContext) {
    if ESR_EL1.read(ESR_EL1::EC) == 0x15 { // SVC call
        syscalls::syscalls(e)
    } else {
        debug_halt("lower_aarch64_synchronous", e);
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(e: &ExceptionContext) {
    irq_handler(e)
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(e: &ExceptionContext)  {
    irq_handler(e)
}

fn debug_halt(string: &'static str, e: &ExceptionContext) {
    debugln!("Kernel Panic ! ");
    debugln!("from {}", string);
    debugln!("Current EL : {}", CurrentEL.get() >> 2);
    debugln!("GPR : {:x?}", e.gpr);
    debugln!("ESR : {:#x?}/{:#x?}", ESR_EL1.read(ESR_EL1::EC), ESR_EL1.get());
    debugln!("FAR : {:#x?}", FAR_EL1.get());
    debugln!("ELR : {:#x?}", e.elr_el1);
    debugln!("PSTATE: {:#x?}", SPSR_EL1.get());
    debugln!("SP_EL0: {:#x?}", SP_EL0.get());

    loop {
        nop();
    }
}
