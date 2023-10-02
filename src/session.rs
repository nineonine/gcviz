use std::{collections::VecDeque, error};

use crate::{
    error::VMError,
    gc::init_collector,
    heap::{CellStatus, MemoryCell},
    log::{Log, LogSource, LOG_CAPACITY},
    program::{InstrResult, Program},
    rts_cfg::ProgramRuntimeConfig,
    simulator::Simulator,
    vm::VirtualMachine,
    wsmsg::InfoBlockData,
};

/// Application result type.
pub type SessionResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Program execution session
pub struct Session {
    pub program: Program,
    pub rts_cfg: ProgramRuntimeConfig,
    pub instr_ptr: usize,
    pub logs: VecDeque<Log>,
    pub log_capacity: usize,
    pub vm: VirtualMachine,
    pub log_dest: LogDestination,
}

pub enum LogDestination {
    EventStream,
    Stdout,
}

impl Default for Session {
    fn default() -> Self {
        Session::new(ProgramRuntimeConfig::default())
    }
}

impl Session {
    pub fn new(rts_cfg: ProgramRuntimeConfig) -> Self {
        let vm = VirtualMachine::new(
            rts_cfg.alignment,
            rts_cfg.heap_size,
            init_collector(&rts_cfg.gc_ty),
        );
        Self {
            program: VecDeque::new(),
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            log_dest: LogDestination::Stdout,
            log_capacity: LOG_CAPACITY,
            instr_ptr: 0,
            rts_cfg,
            vm,
        }
    }

    fn enqueue_log(&mut self, log: Log) {
        if self.logs.len() == self.log_capacity {
            self.logs.pop_front();
        }
        self.logs.push_back(log);
    }

    pub fn gen_program(&mut self) -> (Program, ProgramRuntimeConfig) {
        let mut sim = Simulator::new(self.rts_cfg.clone());
        (sim.gen_program(), sim.rts_cfg)
    }

    /// program interpretation step
    pub fn tick(&mut self) -> Result<InstrResult, VMError> {
        if let Some(instruction) = self.program.get(self.instr_ptr) {
            match self.vm.tick(instruction) {
                Ok(instr_result) => {
                    match &instr_result {
                        InstrResult::Allocate { object, addr } => {
                            self.enqueue_log(Log::new(
                                format!("{object} at 0x{addr:X}"),
                                LogSource::ALLOC,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_allocation(
                                &mut self.vm.heap.memory,
                                *addr,
                                object.size(),
                            );
                        }
                        InstrResult::Read { addr, value } => {
                            self.enqueue_log(Log::new(
                                format!("Read value from 0x{addr:X}. Value: {value}"),
                                LogSource::MUT,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_mutator(&mut self.vm.heap.memory, *addr);
                        }
                        InstrResult::Write { addr, value } => {
                            self.enqueue_log(Log::new(
                                format!("Write value {value:} to 0x{addr:X}"),
                                LogSource::MUT,
                                Some(self.instr_ptr),
                            ));
                            Self::visualize_mutator(&mut self.vm.heap.memory, *addr);
                        }
                        InstrResult::GC { stats } => {
                            self.enqueue_log(Log::new(
                                format!("Collect garbage. Stats: {stats:?}"),
                                LogSource::GC,
                                Some(self.instr_ptr),
                            ));
                        }
                    }
                    self.instr_ptr += 1;
                    return Ok(instr_result);
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
        Err(VMError::UnknownError)
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

    pub fn make_info_block(&self) -> InfoBlockData {
        InfoBlockData {
            gc_type: self.rts_cfg.gc_ty.clone(),
            alignment: self.rts_cfg.alignment,
            heap_size: self.rts_cfg.heap_size,
            allocd_objects: self.vm.heap.objects.len(),
            free_memory: self.vm.heap.free_memory(),
        }
    }
}
