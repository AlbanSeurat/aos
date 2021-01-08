use core::slice;
use core::str::from_utf8_unchecked;
use core::time::Duration;
use mmio::timer::VirtualTimer;
use qemu_exit::QEMUExit;
use crate::exceptions::syscalls;
use cortex_a::regs::{ESR_EL1, RegisterReadWrite, RegisterReadOnly};
use shared::exceptions::handlers::ExceptionContext;
use crate::global::UART;
use mmio::io::Reader;
use crate::memory;
use crate::process::Process;
use mmio::HandleType;
use num_traits::FromPrimitive;

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
        2 => syscall_pause(e.gpr.x[0] as u64),
        3 => syscall_halt(),
        4 => syscall_open_handle(e.gpr.x[0], e.gpr.x[1] as *const u8),
        _ => ()
    }
}

unsafe fn syscall_print(c_string: *const u8, len: usize) {
    let string = slice::from_raw_parts(c_string, len);
    print!("{}", from_utf8_unchecked(string));
}

unsafe fn syscall_pause(secs: u64) {
    let duration = Duration::from_secs(secs);
    VirtualTimer::sleep(duration);
}

unsafe fn syscall_halt() {
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_success();
}

unsafe fn syscall_open_handle(handle_type: u64, _c_string: * const u8) {
    let mut process : &mut Process = core::mem::transmute(memory::map::physical::PROG_META_START);
    let mut result : u64 = u64::MAX;
    debugln!("opening handle for '{}' of type {}", process.name, handle_type);
    let fd = process.open_handle(FromPrimitive::from_u64(handle_type).unwrap());
    // TODO: does not work => stack is restore by ASM Code _ should be change for this handler
    llvm_asm!("mov x0, $0" :: "r"(fd));
}