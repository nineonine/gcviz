use std::error;

use crate::{
    frame::{FrameResult, Program},
    gc::collector::{init_collector, GCType},
    heap::MemoryCell,
    log::{Log, LogSource},
    vm::VirtualMachine,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub program: Program,
    pub frame_ptr: usize,
    pub logs: Vec<Log>,
    pub vm: VirtualMachine,
    pub memviz: Vec<MemoryCell>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(alignment: usize, heap_size: usize, gc_ty: &GCType, program: Program) -> Self {
        let vm = VirtualMachine::new(alignment, heap_size, init_collector(gc_ty));
        let memviz = vm.heap.visualize();
        Self {
            running: true,
            vm,
            logs: vec![],
            memviz,
            program,
            frame_ptr: 0,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(frame) = self.program.pop_front() {
            match self.vm.tick(frame) {
                Ok(frame_result) => {
                    match frame_result {
                        FrameResult::AllocResult(object, addr) => {
                            self.logs.push(Log::new(
                                format!("Allocated Object {object:?} at 0x{addr:X}"),
                                LogSource::ALLOC,
                                Some(self.frame_ptr),
                            ));
                        }
                        FrameResult::ReadResult(addr, result) => {
                            self.logs.push(Log::new(
                                format!("Read value from 0x{addr}. Value: 0x{result:X}"),
                                LogSource::MUT,
                                Some(self.frame_ptr),
                            ));
                        }
                        FrameResult::WriteResult(addr, value) => {
                            self.logs.push(Log::new(
                                format!("Write value 0x{value:X} at address 0x{addr:X}"),
                                LogSource::MUT,
                                Some(self.frame_ptr),
                            ));
                        }
                        FrameResult::GCResult(stats) => {
                            self.logs.push(Log::new(
                                format!("Collect garbage. Stats: {stats:?}"),
                                LogSource::GC,
                                Some(self.frame_ptr),
                            ));
                        }
                    }
                    self.frame_ptr += 1;
                    self.memviz = self.vm.heap.visualize();
                }
                Err(e) => {
                    self.logs.push(Log::new(
                        format!("*** ERROR: {e:?}"),
                        LogSource::ERROR,
                        Some(self.frame_ptr),
                    ));
                }
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
