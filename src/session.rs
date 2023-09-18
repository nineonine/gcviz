use std::{collections::VecDeque, error};

use crate::{
    error::VMError,
    frame::{FrameResult, Program},
    gc::{init_collector, GCType},
    heap::{CellStatus, MemoryCell},
    log::{Log, LogSource, LOG_CAPACITY},
    simulator::Parameters,
    vm::VirtualMachine,
};

/// Application result type.
pub type SessionResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Program execution session
pub struct Session {
    pub program: Program,
    pub instr_ptr: usize,
    pub logs: VecDeque<Log>,
    pub log_capacity: usize,
    pub vm: VirtualMachine,

    pub sim_params: Parameters,
    pub log_dest: LogDestination,
}

pub enum LogDestination {
    EventStream,
    Stdout,
}

impl Session {
    pub fn new(
        heap_size: usize,
        alignment: usize,
        gc_ty: &GCType,
        program: Program,
        sim_params: Parameters,
        log_dest: LogDestination,
    ) -> Self {
        let vm = VirtualMachine::new(alignment, heap_size, init_collector(gc_ty));
        Self {
            program,
            vm,
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            log_capacity: LOG_CAPACITY,
            instr_ptr: 0,
            sim_params,
            log_dest,
        }
    }

    fn enqueue_log(&mut self, log: Log) {
        if self.logs.len() == self.log_capacity {
            self.logs.pop_front();
        }
        self.logs.push_back(log);
    }

    /// program interpretation step
    pub fn tick(&mut self) -> Result<(), VMError> {
        if let Some(instruction) = self.program.get(self.instr_ptr) {
            match self.vm.tick(instruction) {
                Ok(instr_result) => {
                    match instr_result {
                        FrameResult::AllocResult(object, addr) => {
                            self.enqueue_log(Log::new(
                                format!("{object} at 0x{addr:X}"),
                                LogSource::ALLOC,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_allocation(
                                &mut self.vm.heap.memory,
                                addr,
                                object.size(),
                            );
                        }
                        FrameResult::ReadResult(addr, result) => {
                            self.enqueue_log(Log::new(
                                format!("Read value from 0x{addr:X}. Value: {result}"),
                                LogSource::MUT,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_mutator(&mut self.vm.heap.memory, addr);
                        }
                        FrameResult::WriteResult(addr, value) => {
                            self.enqueue_log(Log::new(
                                format!("Write value {value:} to 0x{addr:X}"),
                                LogSource::MUT,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_mutator(&mut self.vm.heap.memory, addr);
                        }
                        FrameResult::GCResult(stats) => {
                            self.enqueue_log(Log::new(
                                format!("Collect garbage. Stats: {stats:?}"),
                                LogSource::GC,
                                Some(self.instr_ptr),
                            ));
                        }
                    }
                    self.instr_ptr += 1;
                }
                Err(e) => {
                    let err_log =
                        Log::new(format!("{e:?}"), LogSource::ERROR, Some(self.instr_ptr));
                    match self.log_dest {
                        LogDestination::Stdout => println!("{e:?}"),
                        LogDestination::EventStream => self.enqueue_log(err_log),
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn restart(&mut self) {
        // Reset the state of the application
        self.instr_ptr = 0;
        self.logs.clear();
        self.enqueue_log(Log::new(
            "Program restarted. Hit 'space' to run.".to_string(),
            LogSource::VM,
            Some(self.instr_ptr),
        ));
        self.vm.heap.memory = vec![MemoryCell::new(CellStatus::Free); self.vm.heap.memory.len()];

        // Reinitialize the VM
        let new_collector = self.vm.collector.new_instance();
        self.vm = VirtualMachine::new(
            self.vm.allocator.alignment,
            self.vm.heap.memory.len(),
            new_collector,
        );
    }

    fn visualize_mutator(memory: &mut [MemoryCell], addr: usize) {
        memory[addr] = MemoryCell::new(CellStatus::Used);
    }

    fn visualize_allocation(memory: &mut [MemoryCell], addr: usize, size: usize) {
        for c in memory.iter_mut().skip(addr).take(size) {
            *c = MemoryCell::new(CellStatus::Allocated);
        }
    }
}
