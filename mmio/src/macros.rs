use core::fmt::{Write, Arguments};
use crate::{LOGGER, SCREEN};

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::macros::_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {
        $crate::macros::_debug(format_args!($($arg)*));
        debug!("\n");
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::macros::_print(format_args!($($arg)*));
        print!("\n");
    }
}

#[doc(hidden)]
pub fn _debug(args: Arguments) {
    unsafe {
        LOGGER.write_fmt(args).unwrap();
    }
}

pub fn _print(args: Arguments) {
    unsafe {
        SCREEN.write_fmt(args).unwrap();
    }
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts((p as *const T) as *const u8,core::mem::size_of::<T>())
}
