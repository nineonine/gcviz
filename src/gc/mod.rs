pub mod mark_sweep;
pub mod stats;

use serde::{Deserialize, Serialize};

use self::{mark_sweep::MarkSweep, stats::GCStats};
use crate::{error::VMError, heap::Heap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GCType {
    MarkSweep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GCEvent {
    GCPhase { msg: String },
    MarkObject { addr: usize },
    FreeObject { addr: usize },
}

impl GCEvent {
    fn phase(msg: String) -> Self {
        GCEvent::GCPhase { msg }
    }
}

pub fn init_collector(gc_ty: &GCType) -> Box<dyn GarbageCollector> {
    match gc_ty {
        GCType::MarkSweep => Box::new(MarkSweep::new()),
    }
}

pub trait GarbageCollector: Send {
    fn collect(&self, heap: &mut Heap) -> Result<(GCStats, Vec<GCEvent>), VMError>;
    fn ty(&self) -> GCType;
    fn new_instance(&self) -> Box<dyn GarbageCollector>;
}
