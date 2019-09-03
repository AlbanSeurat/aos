use cortex_a::{barrier, regs::*};

pub mod handlers;

pub unsafe fn set_vbar_el1_checked(vec_base_addr: u64) {
    cortex_a::regs::VBAR_EL1.set(vec_base_addr);
    // Force VBAR update to complete before next instruction.
    barrier::isb(barrier::SY);
}

