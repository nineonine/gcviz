use crate::error::VMError;

use super::collector::GarbageCollector;

pub struct MarkSweep {}

impl MarkSweep {
    pub fn new() -> Self {
        MarkSweep {}
    }
}

impl GarbageCollector for MarkSweep {
    fn collect(&self) -> Result<(), VMError> {
        Ok(())
    }
}
