use crate::memory;
use cortex_a::regs::{SP_EL0, RegisterReadWrite, ELR_EL1, SPSR_EL1};
use cortex_a::asm;
use core::intrinsics::size_of;
use mmio::{Handle, HandleType};
use crate::global::TIMER;
use shared::exceptions::handlers::GPR;

#[repr(C)]
pub struct ProcessContext {
    pub regs: GPR,
    pub state: usize,
    pub ttbr0: usize,
}

impl ProcessContext {
    pub fn new(regs: GPR, state: usize, ttbr0: usize) -> Self {
        return ProcessContext {
            regs,
            state,
            ttbr0,
        }
    }
}

pub struct Process {
    pid: usize,
    handles: [Handle; 1024],
}

impl Process {
    pub fn new(pid: usize) -> Self {
        Process {
            pid,
            handles: [Handle { handle_type : HandleType::NONE}; 1024],
        }
    }

    pub fn open_handle(&mut self, handle_type: HandleType, pointer: usize) -> usize {
        let (pos, handle) = self.handles.iter_mut().enumerate()
            .find( | (s, h)| h.handle_type == HandleType::NONE).unwrap();
        handle.handle_type = handle_type;
        pos
    }

    pub fn close_handle(&mut self, pos: usize) {
        let mut handle = self.handles.get_mut(pos).unwrap();
        handle.handle_type = HandleType::NONE;
    }

}

pub(crate) fn k_create_process() -> ! {

    let process = Process::new(1);
    let bytes = include_bytes!("../../program.img");
    unsafe { core::ptr::copy(bytes as *const u8, memory::map::physical::PROG_START as *mut u8, bytes.len()) };
    unsafe { core::ptr::copy( &process as *const _ as * const u8,
                              memory::map::physical::PROG_META_START as *mut u8, core::mem::size_of::<Process>())};

    println!("JUMP to program at {:x}", memory::map::physical::PROG_START as u64);
    SP_EL0.set(0x0040_0000);
    // Indicate that we "return" to EL0
    SPSR_EL1.write(SPSR_EL1::M::EL0t);
    ELR_EL1.set(memory::map::physical::PROG_START as u64);
    asm::eret();
}