use serde::{Deserialize, Serialize};

use crate::{error::VMError, heap::Heap};

use super::{
    compact, object_marker::ObjectMarker, stats::GCStats, GCEvent, GCType, GarbageCollector,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompactAlgorithm {
    TwoFinger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkCompact {
    algo: CompactAlgorithm,
}

impl Default for MarkCompact {
    fn default() -> Self {
        Self::new(CompactAlgorithm::TwoFinger)
    }
}

impl MarkCompact {
    pub fn new(compact: CompactAlgorithm) -> Self {
        MarkCompact { algo: compact }
    }

    fn compact(&self, heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
        match self.algo {
            CompactAlgorithm::TwoFinger => {
                compact::two_finger::compact(heap, eventlog);
            }
        }
    }
}

impl ObjectMarker for MarkCompact {}

impl GarbageCollector for MarkCompact {
    fn collect(&self, heap: &mut Heap) -> Result<(GCStats, Vec<GCEvent>), VMError> {
        let mut eventlog = vec![GCEvent::phase("MarkCompact: START".to_string())];
        eventlog.push(GCEvent::phase("Mark from roots".to_string()));

        self.mark_from_roots(heap, &mut eventlog);
        eventlog.push(GCEvent::phase("compact".to_string()));

        self.compact(heap, &mut eventlog);
        eventlog.push(GCEvent::phase("MarkCompact: END".to_string()));

        Ok((GCStats::new(), eventlog))
    }

    fn ty(&self) -> GCType {
        GCType::MarkCompact(self.algo.clone())
    }

    fn new_instance(&self) -> Box<dyn GarbageCollector> {
        Box::new(MarkCompact::new(self.algo.clone()))
    }
}
