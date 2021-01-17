use crate::io::{Writer, IoResult};

pub struct SysCall {

}

impl Writer for SysCall {

    /// Display a string
    fn write(&mut self, bytes: &[u8]) -> IoResult<usize> {
        unsafe {
            llvm_asm!("mov x0, $0" :: "r"(bytes.as_ptr()));
            llvm_asm!("mov x1, $0" :: "r"(bytes.len()));
            llvm_asm!("SVC 1")
        }
        Ok(bytes.len())
    }

}

impl SysCall {

    pub fn halt(&self) {
        unsafe {
            llvm_asm!("SVC 2");
        }
    }


}
