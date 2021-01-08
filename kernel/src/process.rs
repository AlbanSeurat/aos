use crate::memory;
use cortex_a::regs::{SP_EL0, RegisterReadWrite, ELR_EL1, SPSR_EL1};
use cortex_a::asm;
use core::intrinsics::size_of;
use mmio::{Handle, HandleType};

pub struct Process {
    pub name: &'static str,
    handles: [Handle; 1024]
}

impl Process {
    pub fn new(name: &'static str) -> Self {
        Process {
            name,
            handles: [Handle { handle_type : HandleType::NONE}; 1024],
        }
    }

    pub fn open_handle(&mut self, handle_type: HandleType) -> usize {
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

    let process = Process::new("program");

    let bytes = include_bytes!("../../program.img");
    println!("copying program from {:p} to {:#x} with len {}", bytes as *const u8, memory::map::physical::PROG_START, bytes.len());
    unsafe { core::ptr::copy(bytes as *const u8, memory::map::physical::PROG_START as *mut u8, bytes.len()) };
    unsafe { core::ptr::copy( &process as *const _ as * const u8, memory::map::physical::PROG_META_START as *mut u8, core::mem::size_of::<Process>())};

    println!("JUMP to program at {:x}", memory::map::physical::PROG_START as u64);
    SP_EL0.set(0x0040_0000);
    // Indicate that we "return" to EL0
    SPSR_EL1.write(SPSR_EL1::M::EL0t);
    ELR_EL1.set(memory::map::physical::PROG_START as u64);
    asm::eret();
}