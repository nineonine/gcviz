use std::error;

use rand::{distributions::Uniform, prelude::Distribution};

use crate::mem::{CellStatus, MemoryCell};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub mem: Vec<MemoryCell>,
}

impl Default for App {
    fn default() -> Self {
        // Generate mock memory
        let memory_size = 1024;
        Self {
            running: true,
            mem: (0..memory_size).map(|_| random_memory_cell()).collect(),
        }
    }
}

fn random_memory_cell() -> MemoryCell {
    let mut rng = rand::thread_rng();
    let cell_status_distribution = Uniform::from(0..3);

    let random_status = match cell_status_distribution.sample(&mut rng) {
        0 => CellStatus::Free,
        1 => CellStatus::Allocated,
        _ => CellStatus::Marked,
    };

    MemoryCell {
        status: random_status,
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
