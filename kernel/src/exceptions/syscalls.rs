use core::slice;
use core::str::from_utf8_unchecked;
use core::time::Duration;
use mmio::timer::PhysicalTimer;
use qemu_exit::QEMUExit;
use crate::exceptions::{syscalls};
use cortex_a::regs::{ESR_EL1, SP, RegisterReadWrite, RegisterReadOnly};
use shared::exceptions::handlers::ExceptionContext;
use crate::global::{UART, BCMDEVICES};
use mmio::io::Reader;
use crate::memory;

pub(crate) unsafe fn reset() {
    let received = UART.read_char().unwrap_or(0) as char;
    match received {
        'r' => llvm_asm!("HVC 1"),
        'h' => syscall_halt(),
        _ => debug!("UART received : {}\n", received),
    }
}

pub(crate) unsafe fn syscalls(e : &ExceptionContext) {
    match ESR_EL1.read(ESR_EL1::ISS) {
        1 => syscall_print(e.gpr.x[0] as *const u8, e.gpr.x[1] as usize),
        2 => syscall_halt(),
        _ => ()
    }
}

unsafe fn syscall_print(c_string: *const u8, len: usize) {
    let string = slice::from_raw_parts(c_string, len);
    print!("{}", from_utf8_unchecked(string));
}


unsafe fn syscall_halt() {
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_success();
}
