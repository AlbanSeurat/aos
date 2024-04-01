use core::arch::global_asm;
use core::fmt::{Debug, Formatter};
use core::fmt;
global_asm!(include_str!("vectors.S"));

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct GPR {
    pub x: [u64; 31],
}

impl Debug for GPR {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.x.len() {
            f.write_fmt(format_args!("x{:02}:{:08x}, ", i, self.x[i])).expect("Debug formatting error");
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ExceptionContext {
    // General Purpose Registers
    pub gpr: GPR,
    pub spsr_el1: u64,
    pub elr_el1: u64,
    pub stack_el0: u64,
    pub stack_el1: u64,

}

