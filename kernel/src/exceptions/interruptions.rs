use crate::global::{BCMDEVICES, TIMER};
use crate::exceptions::{syscalls, debug_halt};
use shared::exceptions::handlers::ExceptionContext;

pub unsafe fn irq_handler(e: &ExceptionContext) -> u64 {
    let source = BCMDEVICES.CORE0_INTERRUPT_SOURCE.get();
    match source {
        0x800 => { println!("timer tick : {}", TIMER.irq_handle(&BCMDEVICES)); },
        0x100 => syscalls::reset(), // UART
        _ => debug_halt("current_elx_irq", e)
    };
    u64::MAX
}