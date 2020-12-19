use crate::logger::Appender;

pub struct SysCall {

}

impl Appender for SysCall {

    /// Display a string
    fn puts(&self, string: &str) {
        unsafe {
            llvm_asm!("mov x0, $0" :: "r"(string.as_ptr()));
            llvm_asm!("mov x1, $0" :: "r"(string.as_bytes().len()));
            llvm_asm!("SVC 1")
        }
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
}
