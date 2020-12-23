use core::fmt;
use core::ops::Deref;
use crate::uart::Uart;
use crate::syscall::SysCall;

pub struct Logger {
    output: Output,
}

pub trait Appender {
    fn puts(&self, _string: &str) {}
}

pub struct NullLogger;
impl Appender for NullLogger {}

/// Possible outputs which the console can store.
pub enum Output {
    None(NullLogger),
    Uart(Uart),
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


impl Deref for Logger {
    type Target = dyn Appender;

    fn deref(&self) -> &Self::Target {
        match &self.output {
            Output::None(i) => i,
            Output::Uart(i) => i,
            Output::Syscall(i ) => i,
        }
    }
}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}