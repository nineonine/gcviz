use crate::{error::VMError, heap::Heap};

use super::{
    object_marker::ObjectMarker,
    stats::GCStats,
    GCEvent, {GCType, GarbageCollector},
};

pub struct MarkSweep {}

impl Default for MarkSweep {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectMarker for MarkSweep {}

impl MarkSweep {
    pub fn new() -> Self {
        MarkSweep {}
    }

    fn sweep(&self, heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
        let mut addresses_to_remove = Vec::new();

        for (addr, obj) in &heap.objects {
            if !obj.header.marked {
                addresses_to_remove.push(*addr);
            }
        }

        for addr in addresses_to_remove {
            let obj_size = heap.objects.get(&addr).unwrap().size(); // Retrieve size before freeing the object
            match heap.free_object(addr) {
                Ok(_) => {
                    eventlog.push(GCEvent::FreeObject {
                        addr,
                        size: obj_size,
                    });
                }
                Err(_e) => panic!("sweep:free_object at {addr:}"),
            }
        }

        heap.merge_free_ranges();
    }
}

impl GarbageCollector for MarkSweep {
    fn collect(&self, heap: &mut Heap) -> Result<(GCStats, Vec<GCEvent>), VMError> {
        let mut eventlog = vec![GCEvent::phase("MarkSweep: START".to_string())];
        eventlog.push(GCEvent::phase("Mark from roots".to_string()));

        self.mark_from_roots(heap, &mut eventlog);
        eventlog.push(GCEvent::phase("sweep".to_string()));

        self.sweep(heap, &mut eventlog);
        eventlog.push(GCEvent::phase("MarkSweep: END".to_string()));

        Ok((GCStats::new(), eventlog))
    }

    fn ty(&self) -> GCType {
        GCType::MarkSweep
    }

    fn new_instance(&self) -> Box<dyn GarbageCollector> {
        Box::new(MarkSweep::new())
    }
}
