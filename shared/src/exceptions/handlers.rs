global_asm!(include_str!("vectors.S"));

#[repr(C)]
pub struct GPR {
    x: [u64; 31],
}

#[repr(C)]
pub struct ExceptionContext {
    // General Purpose Registers
    gpr: GPR,
    spsr_el1: u64,
    elr_el1: u64,
}


/// The default exceptions, invoked for every exceptions type unless the handler
/// is overwritten.
#[no_mangle]
unsafe extern "C" fn default_exception_handler() {
    debugln!("Unexpected exceptions. Halting CPU.");

    loop {
        cortex_a::asm::wfe()
    }
}

// To implement an exceptions handler, overwrite it by defining the respective
// function below.
// Don't forget the #[no_mangle] attribute.
//
// unsafe extern "C" fn current_el0_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn current_el0_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn current_el0_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn current_elx_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn current_elx_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn lower_aarch64_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch64_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn lower_aarch32_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext);
