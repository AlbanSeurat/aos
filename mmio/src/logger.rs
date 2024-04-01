use core::fmt;
use core::ops::{Deref, DerefMut};
use crate::uart::Uart;
use crate::syscall::SysCall;
use crate::FrameBufferConsole;
use crate::io::{Writer, IoResult};

pub struct Logger {
    output: Output,
}

pub struct NullLogger;

impl Writer for NullLogger {
    fn write(&mut self, bytes: &[u8]) -> IoResult<usize> {
        Ok(bytes.len())
    }
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
    type Target = dyn Writer;

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
        self.writes(s);
        Ok(())
    }
}