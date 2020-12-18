


pub unsafe fn enable_irq() {
    llvm_asm!("msr daifclr, #2");
}

pub unsafe fn disable_irq() {
    llvm_asm!("msr daifset, #2");
}


