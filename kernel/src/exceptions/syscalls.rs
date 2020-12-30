use core::slice;
use core::str::from_utf8_unchecked;
use core::time::Duration;
use mmio::timer::VirtualTimer;
use qemu_exit::QEMUExit;


pub(crate) unsafe fn syscall_one(c_string: *const u8, len: usize) {
    let string = slice::from_raw_parts(c_string,len);
    print!("{}", from_utf8_unchecked(string));
}

pub(crate) unsafe fn syscall_two(secs: u64) {
    let duration = Duration::from_secs(secs);
    VirtualTimer::sleep(duration);
}

pub(crate) unsafe fn syscall_three() {
    const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();
    QEMU_EXIT_HANDLE.exit_success();
}