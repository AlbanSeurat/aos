use core::fmt::{Write, Arguments};
use crate::LOGGER;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::macros::_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debugln {
    () => (debug!("\n"));
    ($($arg:tt)*) => {
        $crate::macros::_debug(format_args!($($arg)*));
        debug!("\n");
    }
}

#[doc(hidden)]
pub fn _debug(args: Arguments) {
    unsafe {
        LOGGER.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! log {
    ($w:ident, $($arg:tt)*) => {
       $w.write_fmt(format_args!($($arg)*)).unwrap();
       $w.write_str("\n").unwrap();
    }
}