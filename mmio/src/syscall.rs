use crate::logger::Appender;

pub struct SysCall {
}

impl Appender for SysCall {

    /// Display a string
    fn puts(&self, string: &str) {
        unsafe {
            asm!("mov x0, $0" :: "r"(string.as_ptr()));
            asm!("mov x1, $0" :: "r"(string.as_bytes().len()));
            asm!("SVC 1")
        }
    }

}
