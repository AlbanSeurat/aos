global_asm!(include_str!("vectors.S"));

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GPR {
    pub x: [u64; 31],
}

#[repr(C)]
#[derive(Debug)]
pub struct ExceptionContext {
    // General Purpose Registers
    pub gpr: GPR,
    pub spsr_el1: u64,
    pub elr_el1: u64,
    pub stack: u64,
}

