use crate::error::VMError;

use super::collector::GarbageCollector;

struct MarkSweep {}

impl GarbageCollector for MarkSweep {
    fn collect(&self) -> Result<(), VMError> {
        Ok(())
    }
}
