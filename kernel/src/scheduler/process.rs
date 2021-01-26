global_asm!(include_str!("context.S"));
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::intrinsics::{size_of, const_allocate};

use cortex_a::asm;
use cortex_a::regs::{ELR_EL1, RegisterReadWrite, SP_EL0, SPSR_EL1, TTBR0_EL1, ELR_EL3, SP, SP_EL1};

use shared::exceptions::handlers::GPR;

use crate::memory;
use shared::memory::mmu::{ArchTranslationTable, setup_dyn_user_tables, switch_user_tables, VIRTUAL_ADDR_START};
use shared::memory::mapping::Descriptor;
use crate::scheduler::PROG_START;
use crate::scheduler::process::ProcessState::{Sleep, Running};
use core::cell::Cell;
use cortex_a::asm::eret;
use crate::global::{SCHEDULER, BCMDEVICES, TIMER};
use core::fmt::{Debug, Formatter};
use core::{fmt, slice};
use alloc::fmt::format;
use core::str::from_utf8_unchecked;

extern "C" {
    #[inline]
    fn __restore_and_eret(regs: usize) -> !;
}

#[repr(C)]
#[derive(Debug)]
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
            stack,
        };
    }
}

impl Default for ProcessContext {
    fn default() -> Self {
        ProcessContext {
            regs: Default::default(),
            state: 0,
            eret_addr: PROG_START as u64,
            stack: 0x0040_0000,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ProcessState {
    Running,
    Sleep,
}

pub struct Process {
    pub tlb: ArchTranslationTable,
    pub pid: u16,
    state: ProcessState,
    context: ProcessContext,
}

impl Debug for Process {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = format!("Process with mmu {:x} - ", self.tlb.phys_base_addr());
        f.debug_tuple(string.as_str())
            .field(&self.pid)
            .field(&self.state)
            .field(&self.context)
            .finish()
    }
}

impl Process {
    pub fn new(pid: u16) -> Self {
        Process {
            tlb: ArchTranslationTable::new(),
            pid,
            state: Sleep,
            context: Default::default(),
        }
    }

    pub fn init_local_tlb(&mut self, descriptors: &Vec<Descriptor>) {
        let desc_iter = descriptors.iter();
        setup_dyn_user_tables(&desc_iter, &mut self.tlb, self.pid);
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
        if self.state == Running {
            self.state = Sleep;
            self.context = ProcessContext::new(*gpr, state, eret_addr, stack);
        }
    }

    pub fn restore(&mut self, stack: u64) {
        self.state = Running;
        switch_user_tables(self.pid, self.tlb.phys_base_addr() as u64);
        SPSR_EL1.set(self.context.state);
        ELR_EL1.set(self.context.eret_addr);
        SP_EL0.set(self.context.stack);
        SPSR_EL1.write(SPSR_EL1::M::EL0t);
        SP.set(stack);
        unsafe { __restore_and_eret(self.context.regs.x.as_ptr() as usize) };
    }
}

pub(crate) fn create_tmp_init_program() -> &'static mut Process {
    let process = unsafe { SCHEDULER.create_process() };
    let bytes = include_bytes!("../../../program.img");
    unsafe { core::ptr::copy(bytes as *const u8, PROG_START as *mut u8, bytes.len()) };
    process
}

pub(crate) fn create_init_program() -> &'static mut Process {
    let process = unsafe { SCHEDULER.create_process() };
    let bytes = include_bytes!("../../../init.img");
    unsafe { core::ptr::copy(bytes as *const u8, PROG_START as *mut u8, bytes.len()) };
    //process.run();
    process
}