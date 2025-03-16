use core::arch::asm;
use core::slice;
use core::str::from_utf8_unchecked;
use qemu_exit::QEMUExit;

use aarch64_cpu::registers::{ESR_EL1, Readable, SP};
use shared::exceptions::handlers::ExceptionContext;
use crate::global::{SCHEDULER, UART};
use mmio::io::Reader;
use crate::scheduler::process::Process;
use crate::scheduler::PROG_START;

pub(crate) unsafe fn reset() {
    let received = UART.read_char().unwrap_or(0) as char;
    match received {
        'r' => asm!("HVC 1"),
        'h' => syscall_halt(),
        _ => debug!("UART received : {}\n", received),
    }
}

pub(crate) unsafe fn syscalls(e : &ExceptionContext) {
    match ESR_EL1.read(ESR_EL1::ISS) {
        1 => syscall_print(e.gpr.x[0] as *const u8, e.gpr.x[1] as usize),
        2 => syscall_halt(),
        3 => syscall_sleep(e.gpr.x[0], e),
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

unsafe fn syscall_sleep(ms: u64, e: &ExceptionContext) {
    SCHEDULER.sleep(core::ptr::read((PROG_START - 0x1000) as *const u16), ms, e);
}