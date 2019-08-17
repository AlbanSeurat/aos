use core::fmt::{Write, Arguments};
use crate::kernel::UART;


#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::kernel::macros::_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debugln {
    () => (debug!("\n"));
    ($($arg:tt)*) => {
        $crate::kernel::macros::_debug(format_args!($($arg)*));
        debug!("\n");
    }
}

#[doc(hidden)]
pub fn _debug(args: Arguments) {

    // TODO : should be replace by a real lock (once we have multiple thread)
    unsafe {
        UART.write_fmt(args).unwrap();
    }

}