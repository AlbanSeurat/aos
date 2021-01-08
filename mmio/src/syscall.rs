use crate::io::{Writer, IoResult};
use crate::HandleType;
use num_traits::ToPrimitive;


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

    pub fn sleep(&self, secs: u64) {
        unsafe {
            llvm_asm!("mov x0, $0" :: "r"(secs));
            llvm_asm!("SVC 2");
        }
    }

    pub fn halt(&self) {
        unsafe {
            llvm_asm!("SVC 3");
        }
    }

    pub fn open(&self, handle_type: HandleType) -> u64 {
        let mut result = u64::MAX;
        let t = ToPrimitive::to_usize(&handle_type).unwrap();
        unsafe {
            llvm_asm!("mov x0, $0" :: "r"(t));
            llvm_asm!("SVC 4");
            llvm_asm!("mov $0, x0" : "=r"(result));
        }
        result
    }
}
