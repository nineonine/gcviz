use crate::{
    error::VMError,
    heap::Heap,
    object::{Address, Field, ObjAddr},
};

use super::{
    stats::GCStats,
    GCEvent, {GCType, GarbageCollector},
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

    fn mark_from_roots(&self, heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
        // Clear all existing marks
        for obj in heap.objects.values_mut() {
            obj.header.marked = false;
        }

        // Mark objects starting from roots
        let roots: Vec<usize> = heap.roots.iter().cloned().collect();
        for root in roots {
            self.mark(&root, heap, eventlog);
        }
    }

    fn mark(&self, addr: &ObjAddr, heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
        let mut stack = Vec::new();
        stack.push(*addr);

        while let Some(current_addr) = stack.pop() {
            if let Some(obj) = heap.objects.get_mut(&current_addr) {
                if obj.header.marked {
                    continue; // Already marked, no need to continue
                }

                obj.header.marked = true;
                eventlog.push(GCEvent::MarkObject {
                    addr: current_addr,
                    size: obj.size(),
                });

                for field in &obj.fields {
                    if let Field::Ref {
                        addr: Address::Ptr(addr),
                    } = field
                    {
                        stack.push(*addr);
                    }
                }
            }
        }
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
