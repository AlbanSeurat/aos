use crate::global::{BCMDEVICES, SCHEDULER};
use crate::exceptions::{syscalls, debug_halt};
use shared::exceptions::handlers::ExceptionContext;
use tock_registers::interfaces::Readable;


pub unsafe fn irq_handler(e: &ExceptionContext) {
    let source = BCMDEVICES.CORE0_INTERRUPT_SOURCE.get();
    match source {
        2 => SCHEDULER.schedule(e),
        0x100 => syscalls::reset(), // UART
        _ => debug_halt("current_elx_irq", e)
    };
}