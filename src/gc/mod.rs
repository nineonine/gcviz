pub mod mark_sweep;
pub mod stats;

use self::{mark_sweep::MarkSweep, stats::GCStats};
use crate::error::VMError;

pub enum GCType {
    MarkSweep,
}

pub fn init_collector(gc_ty: &GCType) -> Box<dyn GarbageCollector> {
    match gc_ty {
        GCType::MarkSweep => Box::new(MarkSweep::new()),
    }
}

pub trait GarbageCollector {
    fn collect(&self) -> Result<GCStats, VMError>;
    fn ty(&self) -> GCType;
    fn new_instance(&self) -> Box<dyn GarbageCollector>;
}
