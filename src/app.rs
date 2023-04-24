use std::error;

use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    gc::collector::GarbageCollector,
    heap::{CellStatus, MemoryCell},
    vm::VirtualMachine,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub logs: Vec<(&'a str, &'a str)>,
    pub vm: VirtualMachine,
    pub memviz: Vec<MemoryCell>,
}

fn random_memory_cell() -> MemoryCell {
    let mut rng = rand::thread_rng();
    let cell_status_distribution = Uniform::from(0..4);

    let random_status = match cell_status_distribution.sample(&mut rng) {
        0 => CellStatus::Freed,
        1 => CellStatus::Allocated,
        2 => CellStatus::Used,
        _ => CellStatus::Marked,
    };

    MemoryCell {
        status: random_status,
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(alignment: usize, heap_size: usize, gc: Box<dyn GarbageCollector>) -> Self {
        Self {
            running: true,
            vm: VirtualMachine::new(alignment, heap_size, gc),
            logs: vec![],
            memviz: (0..1024).map(|_| random_memory_cell()).collect(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
