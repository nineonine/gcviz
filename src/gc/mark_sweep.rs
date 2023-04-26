use crate::error::VMError;

use super::{collector::GarbageCollector, stats::GCStats};

pub struct MarkSweep {}

impl Default for MarkSweep {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkSweep {
    pub fn new() -> Self {
        MarkSweep {}
    }
}

impl GarbageCollector for MarkSweep {
    fn collect(&self) -> Result<GCStats, VMError> {
        Ok(GCStats::new())
    }
}
