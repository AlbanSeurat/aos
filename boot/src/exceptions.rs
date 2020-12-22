use shared::exceptions::set_vbar_el1_checked;
use shared::exceptions::handlers::ExceptionContext;
use cortex_a::regs::{ESR_EL1, FAR_EL1, SPSR_EL1, RegisterReadWrite, RegisterReadOnly};
use qemu_exit::QEMUExit;
use cortex_a::barrier;

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
unsafe extern "C" fn lower_aarch64_synchronous(e : &ExceptionContext) {
    debugln!("Synchronous exception lower EL");
    debug_halt(e);
}

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &ExceptionContext) {
    debugln!("Synchronous exception current EL");
    debug_halt(e);
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(e: &ExceptionContext) {
    debugln!("Current IRQ handling");
    debug_halt(e);
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(e: &ExceptionContext) {
    debugln!("Lower aarch64 IRQ handling");
    debug_halt(e);
}


fn debug_halt(e: &ExceptionContext) {
    debugln!("Kernel Panic ! ");
    debugln!("GPR : {:x?}", e.gpr);
    debugln!("ESR : {:#x?}/{:#x?}", ESR_EL1.read(ESR_EL1::EC), ESR_EL1.get());
    debugln!("FAR : {:#x?}", FAR_EL1.get());
    debugln!("ELR : {:#x?}", e.elr_el1);
    debugln!("PSTATE: {:#x?}", SPSR_EL1.get());

    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_failure();
}