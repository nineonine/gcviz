use crate::error::VMError;

use super::{
    stats::GCStats,
    {GCType, GarbageCollector},
};

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

    fn ty(&self) -> GCType {
        GCType::MarkSweep
    }
}
