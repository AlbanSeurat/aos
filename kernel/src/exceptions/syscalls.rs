use core::slice;
use core::str::from_utf8_unchecked;
use core::time::Duration;
use mmio::timer::VirtualTimer;
use qemu_exit::QEMUExit;
use crate::exceptions::{syscalls};
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

pub(crate) unsafe fn syscalls(e : &ExceptionContext) -> u64 {
    match ESR_EL1.read(ESR_EL1::ISS) {
        1 => syscall_print(e.gpr.x[0] as *const u8, e.gpr.x[1] as usize),
        2 => syscall_pause(e.gpr.x[0] as u64),
        3 => { syscall_halt(); u64::MAX },
        4 => syscall_open_handle(e.gpr.x[0], e.gpr.x[1] as *const u8),
        _ => u64::MAX
    }
}

unsafe fn syscall_print(c_string: *const u8, len: usize) -> u64 {
    let string = slice::from_raw_parts(c_string, len);
    print!("{}", from_utf8_unchecked(string));
    u64::MAX
}

unsafe fn syscall_pause(secs: u64) -> u64 {
    let duration = Duration::from_secs(secs);
    VirtualTimer::sleep(duration);
    u64::MAX
}

unsafe fn syscall_halt() {
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_success();
}

unsafe fn syscall_open_handle(handle_type: u64, _c_string: * const u8) -> u64 {
    let mut process : &mut Process = core::mem::transmute(memory::map::physical::PROG_META_START);
    let mut result : u64 = u64::MAX;
    let fd = process.open_handle(FromPrimitive::from_u64(handle_type).unwrap());
    debugln!("opening handle for '{}' of type {} => index {}", process.name, handle_type, fd);
    llvm_asm!("mov x1, $0" :: "r"(1024));
    fd as u64
}