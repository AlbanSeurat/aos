global_asm!(include_str!("context.S"));
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::intrinsics::size_of;

use cortex_a::asm;
use cortex_a::regs::{ELR_EL1, RegisterReadWrite, SP_EL0, SPSR_EL1, TTBR0_EL1, ELR_EL3};

use shared::exceptions::handlers::GPR;

use crate::global::{SCHEDULER, TIMER};
use crate::memory;
use shared::memory::mmu::{ArchTranslationTable, setup_dyn_user_tables, switch_user_tables};
use shared::memory::mapping::Descriptor;
use crate::scheduler::PROG_START;
use crate::scheduler::process::ProcessState::{Sleep, Running};
use core::cell::Cell;
use cortex_a::asm::eret;

extern "C" {
    #[inline]
    fn __restore_and_eret(regs: usize) -> !;
}

#[repr(C)]
pub struct ProcessContext {
    pub(crate) regs: GPR,
    pub(crate) state: u64,
    pub(crate) eret_addr: u64,
    pub(crate) stack: u64,
}

impl ProcessContext {
    pub fn new(regs: GPR, state: u64, eret_addr: u64, stack: u64) -> Self {
        return ProcessContext {
            regs,
            state,
            eret_addr,
            stack
        }
    }
}

impl Default for ProcessContext {
    fn default() -> Self {
        ProcessContext {
            regs: Default::default(),
            state: 0,
            eret_addr: PROG_START as u64,
            stack: 0x0040_0000
        }
    }
}

#[derive(PartialEq)]
pub enum ProcessState {
    Running,
    Sleep
}

pub struct Process {
    tlb: ArchTranslationTable,
    pid: u16,
    state: ProcessState,
    context: ProcessContext
}

impl Process {
    pub fn new(pid: u16) -> Self {
        Process {
            tlb: ArchTranslationTable::new(),
            pid,
            state: Sleep,
            context: Default::default()
        }
    }

    pub fn init_local_tlb(&mut self, descriptors: &Vec<Descriptor>) {
        let desc_iter = descriptors.iter();
        self.tlb.map_descriptors(&desc_iter);
        setup_dyn_user_tables(&desc_iter, &mut self.tlb);
        unsafe { print!("MMU Program mapping : \n{}", self.tlb); }
    }

    pub fn run(&mut self) {
        self.state = Running;
        println!("START PROCESS PID {} at {:x}", self.pid, PROG_START as u64);
        SP_EL0.set(0x0040_0000);
        // Indicate that we "return" to EL0
        SPSR_EL1.write(SPSR_EL1::M::EL0t);
        ELR_EL1.set(PROG_START as u64);
        asm::eret();
    }

    pub fn is_running(&self) -> bool {
        self.state == Running
    }

    pub fn sleep(&mut self, gpr: &GPR, state: u64, eret_addr: u64, stack: u64) {
        self.state = Sleep;
        self.context = ProcessContext::new(*gpr, state, eret_addr, stack);
    }

    pub fn restore(&mut self) {
        self.state = Running;
        switch_user_tables(self.tlb.phys_base_addr() as u64);
        SPSR_EL1.set(self.context.state);
        ELR_EL1.set(self.context.eret_addr);
        SP_EL0.set(self.context.stack);
        unsafe { __restore_and_eret(self.context.regs.x.as_ptr() as usize) };
    }
}

pub(crate) fn create_tmp_init_program() -> () {
    let process = unsafe { SCHEDULER.create_process() };
    let bytes = include_bytes!("../../../program.img");
    unsafe { core::ptr::copy(bytes as *const u8, PROG_START as *mut u8, bytes.len()) };
}

pub(crate) fn create_init_program() -> () {

    let mut process = unsafe { SCHEDULER.create_process() };
    let bytes = include_bytes!("../../../program.img");
    unsafe { core::ptr::copy(bytes as *const u8, PROG_START as *mut u8, bytes.len()) };
    process.run();
}