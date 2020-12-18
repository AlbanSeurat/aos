use core::slice;
use core::str::from_utf8_unchecked;
use core::time::Duration;
use mmio::timer::Timer;
use crate::memory;


pub(crate) unsafe fn syscall_one(c_string: *const u8, len: usize) {
    let string = slice::from_raw_parts(c_string,len);
    debug!("{}", from_utf8_unchecked(string));
}

pub(crate) unsafe fn syscall_two(secs: u64) {
    let duration = Duration::from_secs(secs);
    Timer::sleep(duration);
}