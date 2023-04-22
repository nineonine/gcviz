use std::error;

use rand::{distributions::Uniform, prelude::Distribution};

use crate::heap::{CellStatus, MemoryCell};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub mem: Vec<MemoryCell>,
    pub logs: Vec<(&'a str, &'a str)>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            mem: (0..1024).map(|_| random_memory_cell()).collect(),
            logs: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
                ("Event5", "INFO"),
                ("Event6", "INFO"),
                ("Event7", "WARNING"),
                ("Event8", "INFO"),
                ("Event9", "INFO"),
                ("Event10", "INFO"),
                ("Event11", "CRITICAL"),
                ("Event12", "INFO"),
                ("Event13", "INFO"),
                ("Event14", "INFO"),
                ("Event15", "INFO"),
                ("Event16", "INFO"),
                ("Event17", "ERROR"),
                ("Event18", "ERROR"),
                ("Event19", "INFO"),
                ("Event20", "INFO"),
                ("Event21", "WARNING"),
                ("Event22", "INFO"),
                ("Event23", "INFO"),
                ("Event24", "WARNING"),
                ("Event25", "INFO"),
                ("Event26", "INFO"),
            ],
        }
    }
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
