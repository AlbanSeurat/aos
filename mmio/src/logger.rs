use core::fmt;
use core::ops::{Deref, DerefMut};
use crate::uart::Uart;
use crate::syscall::SysCall;
use crate::FrameBufferConsole;
use core::borrow::BorrowMut;

pub struct Logger {
    output: Output,
}

pub trait Appender {
    fn puts(&mut self, _string: &str);
}

pub struct NullLogger;

impl Appender for NullLogger {
    fn puts(&mut self, _string: &str) {}
}

/// Possible outputs which the console can store.
pub enum Output {
    None(NullLogger),
    Uart(Uart),
    ScreenConsole(FrameBufferConsole),
    Syscall(SysCall),
}

impl Logger {
    pub const fn new() -> Logger {
        Logger {
            output: Output::None(NullLogger {}),
        }
    }

    pub fn appender(&mut self, x: Output) {
        self.output = x;
    }
}

impl From<Uart> for Output {
    fn from(instance: Uart) -> Self {
        Output::Uart(instance)
    }
}

impl From<SysCall> for Output {
    fn from(instance: SysCall) -> Self {
        Output::Syscall(instance)
    }
}

impl From<FrameBufferConsole> for Output {
    fn from(instance: FrameBufferConsole) -> Self {
        Output::ScreenConsole(instance)
    }
}

impl Deref for Logger {
    type Target = dyn Appender;

    fn deref(&self) -> &Self::Target {
        match &self.output {
            Output::None(i) => i,
            Output::Uart(i) => i,
            Output::ScreenConsole(i) => i,
            Output::Syscall(i) => i,
        }
    }
}

impl DerefMut for Logger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.output {
            Output::None(i) => i,
            Output::Uart(i) => i,
            Output::ScreenConsole(i) => i,
            Output::Syscall(i) => i,
        }
    }
}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}