use cortex_a::regs::CNTFRQ_EL0;
use register::cpu::RegisterReadOnly;

pub struct Timer {

}

impl Timer {
    pub fn init() {
        debugln!("current timer values {}", CNTFRQ_EL0.get() as u64);
    }
}