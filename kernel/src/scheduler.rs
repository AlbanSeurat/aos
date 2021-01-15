use alloc::vec::Vec;

use process::Process;
use crate::memory::descriptors::PROGRAM_VIRTUAL_LAYOUT;
use shared::memory::mapping::{Descriptor, Mapping, Translation, AttributeFields, MemAttributes, AccessPermissions};
use core::ops::RangeInclusive;
use crate::global::{TIMER, BCMDEVICES};
use shared::exceptions::handlers::ExceptionContext;
use crate::scheduler::process::ProcessState::Running;

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
        let mut process = Process::new(self.pid);
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
        process.init_local_tlb(&descriptors);
        self.pid = self.pid + 1;
        self.processes.push(process);
        self.processes.last_mut().expect("created process not working properly")
    }

    pub unsafe fn schedule(&mut self, e: &ExceptionContext) {
        TIMER.irq_handle(&BCMDEVICES);

        let (pos, process) = self.processes.iter_mut()
            .enumerate().find( | (pos, p)| p.is_running())
            .expect("At least on process must run");
        process.sleep(&e.gpr, e.spsr_el1, e.elr_el1, e.stack);

        let nb_processes = self.processes.len();
        let mut restaured = self.processes
            .get_mut( if pos == nb_processes - 1 { 0 } else { pos + 1} )
            .expect("At least on process should have be created");

        restaured.restore()
    }
}
