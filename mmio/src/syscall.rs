use crate::io::{Writer, IoResult};
use core::arch::asm;

pub struct SysCall {

}

impl Writer for SysCall {

    /// Display a string
    fn write(&mut self, bytes: &[u8]) -> IoResult<usize> {
        unsafe {
            asm!("SVC 1", in("x0") bytes.as_ptr(), in("x1") bytes.len());
        }
        Ok(bytes.len())
    }

}

impl SysCall {

    pub fn halt(&self) {
        unsafe {
            asm!("SVC 2");
        }
    }


}
