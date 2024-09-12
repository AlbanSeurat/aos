use alloc::vec::Vec;

use process::Process;
use crate::memory::descriptors::PROGRAM_VIRTUAL_LAYOUT;
use shared::memory::mapping::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use shared::exceptions::handlers::ExceptionContext;
use crate::scheduler::process::ProcessState::Running;
use crate::global::PTIMER;
use aarch64_cpu::registers::{CNTV_TVAL_EL0, Readable};
use shared::memory::mmu::VIRTUAL_ADDR_START;
use core::str::from_utf8_unchecked;
use core::slice;

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

    pub fn create_process(&mut self) -> &mut Process {
        let process = Process::new(self.pid + 1);
        let mut descriptors = PROGRAM_VIRTUAL_LAYOUT.to_vec();
        descriptors.push(Descriptor {
            virtual_range: || RangeInclusive::new(PROG_START, PROG_END - 1),
            map : Mapping {
                translation: Translation::Offset(0x100_0000 * self.pid as usize),
                attribute_fields: AttributeFields {
                    mem_attributes: MemAttributes::CacheableDRAM,
                    acc_perms: AccessPermissions::ReadWriteUser,
                    execute_never: false,
                },
            },
        });
        self.pid = self.pid + 1;
        self.processes.push(process);
        let created_process = self.processes.last_mut().expect("created process not working properly");
        created_process.init_local_tlb(&descriptors);
        created_process
    }

    pub unsafe fn schedule(&mut self, e: &ExceptionContext) {
        PTIMER.reset_counter();

        self.processes.iter_mut()
            .find( | p | p.is_running())
            .map_or(0, | p |
                { p.sleep(&e.gpr, e.spsr_el1, e.elr_el1, e.stack_el0); return 0; });

        let nb_processes = self.processes.len();
        let restaured = self.processes.get_mut(CNTV_TVAL_EL0.get() as usize % nb_processes)
            .expect("process not found");
        restaured.restore(e.stack_el1);
    }
}
