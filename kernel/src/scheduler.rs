use alloc::vec::Vec;
use core::intrinsics::{copy, size_of, transmute};

use process::Process;
use crate::memory::descriptors::PROGRAM_VIRTUAL_LAYOUT;
use shared::memory::mapping::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use aarch64_cpu::asm::wfi;
use shared::exceptions::handlers::ExceptionContext;
use crate::global::TIMER;
use aarch64_cpu::registers::{CNTV_TVAL_EL0, Readable};

pub mod process;

pub const PROG_START: usize = 0x0020_0000;
pub const PROG_END:   usize = 0x0040_0000;

pub struct Scheduler {
    processes: Vec<Process>,
    pid: u16
}

impl Scheduler {
    pub const fn new () -> Self {
        Scheduler {
            processes: Vec::new(),
            pid: 0,
        }
    }

    pub fn create_process(&mut self, bytes: &[u8]) {
        let current_pid = self.pid + 1;
        let mut descriptors = PROGRAM_VIRTUAL_LAYOUT.to_vec();
        descriptors.push(Descriptor {
            virtual_range: || RangeInclusive::new(PROG_START, PROG_END - 1),
            map : Mapping {
                translation: Translation::Offset(0x100_0000 * current_pid as usize),
                attribute_fields: AttributeFields {
                    mem_attributes: MemAttributes::CacheableDRAM,
                    acc_perms: AccessPermissions::ReadWriteUser,
                    execute_never: false,
                },
            },
        });
        descriptors.push(Descriptor {
            virtual_range: || RangeInclusive::new(PROG_START - 0x1000, PROG_START - 1),
            map : Mapping {
                translation: Translation::Offset(0x100_0000 * current_pid as usize - 0x1000),
                attribute_fields: AttributeFields {
                    mem_attributes: MemAttributes::CacheableDRAM,
                    acc_perms: AccessPermissions::ReadWriteKernel,
                    execute_never: true,
                },
            },
        });
        let process = Process::new(current_pid);
        self.processes.push(process); // move the process from stack to heap and allow to have several table entry ...
        let created_process = self.processes.last_mut().expect("created process not working properly");
        created_process.init_local_tlb(&descriptors);
        unsafe {
            copy(bytes.as_ptr(), PROG_START as *mut u8, bytes.len());
            core::ptr::write((PROG_START - 0x1000) as * mut u16, created_process.pid); // todo : write the whole process in the zone (size > 0x1000)
        }
        self.pid = current_pid;
    }

    pub unsafe fn schedule(&mut self, e: &ExceptionContext) {

        TIMER.reset_counter();

        match self.processes.iter_mut()
            .find(|p| p.is_running()) {
            Some(p) => p.pause(&e.gpr, e.spsr_el1, e.elr_el1, e.stack_el0),
            _ => {}
        }

        let nb_processes = self.processes.len();
        match self.processes.get_mut(CNTV_TVAL_EL0.get() as usize % nb_processes) {
            Some(p ) => p.restore(e.stack_el1),
            None => wfi()
        }
    }


    pub fn sleep(&mut self, pid: u16, ms: u64, e: &ExceptionContext) {
        match self.processes.iter_mut()
            .find(|p| p.pid == pid) {
            Some(p) => p.pause(&e.gpr, e.spsr_el1, e.elr_el1, e.stack_el0),
            _ => {}
        }
    }
}
