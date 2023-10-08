use log::debug;

use crate::{
    error::VMError,
    heap::Heap,
    object::{Address, Field, ObjAddr},
};

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

    fn mark_from_roots(&self, heap: &mut Heap) {
        // Clear all existing marks
        for obj in heap.objects.values_mut() {
            obj.header.marked = false;
        }

        // Mark objects starting from roots
        let roots: Vec<usize> = heap.roots.iter().cloned().collect();
        for root in roots {
            self.mark(&root, heap);
        }
    }

    fn mark(&self, addr: &ObjAddr, heap: &mut Heap) {
        let mut stack = Vec::new();
        stack.push(*addr);

        while let Some(current_addr) = stack.pop() {
            if let Some(obj) = heap.objects.get_mut(&current_addr) {
                if obj.header.marked {
                    continue; // Already marked, no need to continue
                }

                obj.header.marked = true;

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

    fn sweep(&self, heap: &mut Heap) {
        let mut addresses_to_remove = Vec::new();

        for (addr, obj) in &heap.objects {
            if !obj.header.marked {
                addresses_to_remove.push(*addr);
            }
        }

        for addr in addresses_to_remove {
            match heap.free_object(addr) {
                Ok(_) => {}
                Err(_e) => panic!("sweep:free_object at {addr:}"),
            }
        }

        heap.merge_free_ranges();
        debug!("____GC objects AFTER {:?}", heap.objects);
        debug!("____ GC free ranges AFTER {:?}", heap.free_list);
    }
}

impl GarbageCollector for MarkSweep {
    fn collect(&self, heap: &mut Heap) -> Result<GCStats, VMError> {
        self.mark_from_roots(heap);
        self.sweep(heap);
        Ok(GCStats::new())
    }

    fn ty(&self) -> GCType {
        GCType::MarkSweep
    }

    fn new_instance(&self) -> Box<dyn GarbageCollector> {
        Box::new(MarkSweep::new())
    }
}
