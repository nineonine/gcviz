use std::{collections::VecDeque, error};

use crate::{
    frame::{FrameResult, Program},
    gc::{init_collector, GCType},
    log::{Log, LogSource, LOG_CAPACITY},
    simulator::Parameters,
    ui::heap::{reset_highlights, visualize_allocation, visualize_mutator, CellStatus, MemoryCell},
    vm::VirtualMachine,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub program: Program,
    pub program_paused: bool,
    pub eval_next_frame: bool,
    pub frame_ptr: usize,
    pub logs: VecDeque<Log>,
    pub log_capacity: usize,
    pub vm: VirtualMachine,

    pub sim_params: Parameters,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(
        heap_size: usize,
        alignment: usize,
        gc_ty: &GCType,
        program: Program,
        sim_params: Parameters,
    ) -> Self {
        let vm = VirtualMachine::new(alignment, heap_size, init_collector(gc_ty));
        Self {
            running: true,
            program,
            program_paused: false,
            eval_next_frame: false,
            vm,
            logs: VecDeque::with_capacity(LOG_CAPACITY),
            log_capacity: LOG_CAPACITY,
            frame_ptr: 0,
            sim_params,
        }
    }

    fn enqueue_log(&mut self, log: Log) {
        if self.logs.len() == self.log_capacity {
            self.logs.pop_front();
        }
        self.logs.push_back(log);
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.program_paused && !self.eval_next_frame {
            return;
        }
        reset_highlights(&mut self.vm.heap.memory);
        if let Some(frame) = self.program.get(self.frame_ptr) {
            match self.vm.tick(frame) {
                Ok(frame_result) => {
                    match frame_result {
                        FrameResult::AllocResult(object, addr) => {
                            self.enqueue_log(Log::new(
                                format!("{object} at 0x{addr:X}"),
                                LogSource::ALLOC,
                                Some(self.frame_ptr),
                            ));
                            visualize_allocation(&mut self.vm.heap.memory, addr, object.size());
                        }
                        FrameResult::ReadResult(addr, result) => {
                            self.enqueue_log(Log::new(
                                format!("Read value from 0x{addr:X}. Value: {result}"),
                                LogSource::MUT,
                                Some(self.frame_ptr),
                            ));
                            visualize_mutator(&mut self.vm.heap.memory, addr);
                        }
                        FrameResult::WriteResult(addr, value) => {
                            self.enqueue_log(Log::new(
                                format!("Write value {value:} to 0x{addr:X}"),
                                LogSource::MUT,
                                Some(self.frame_ptr),
                            ));
                            visualize_mutator(&mut self.vm.heap.memory, addr);
                        }
                        FrameResult::GCResult(stats) => {
                            self.enqueue_log(Log::new(
                                format!("Collect garbage. Stats: {stats:?}"),
                                LogSource::GC,
                                Some(self.frame_ptr),
                            ));
                        }
                    }
                    self.frame_ptr += 1;
                }
                Err(e) => {
                    self.enqueue_log(Log::new(
                        format!("{e:?}"),
                        LogSource::ERROR,
                        Some(self.frame_ptr),
                    ));
                }
            }
        }
        self.eval_next_frame = false;
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn restart(&mut self) {
        // Reset the state of the application
        self.frame_ptr = 0;
        self.logs.clear();
        self.enqueue_log(Log::new(
            "Program restarted. Hit 'space' to run.".to_string(),
            LogSource::VM,
            Some(self.frame_ptr),
        ));
        self.vm.heap.memory = vec![MemoryCell::new(CellStatus::Freed); self.vm.heap.memory.len()];

        // Reinitialize the VM
        let new_collector = self.vm.collector.new_instance();
        self.vm = VirtualMachine::new(
            self.vm.allocator.alignment,
            self.vm.heap.memory.len(),
            new_collector,
        );
    }
}
