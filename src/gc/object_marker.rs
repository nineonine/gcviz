use crate::{
    heap::Heap,
    object::{Address, Field, ObjAddr},
};

use super::GCEvent;

pub trait ObjectMarker {
    fn mark_from_roots(&self, heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
        // Clear all existing marks
        // Current MarkSweep impl needs this
        // TODO: either move it into MarkSweep or just do for MarkSweep only
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
}

pub fn is_marked(heap: &Heap, addr: ObjAddr) -> bool {
    if let Some(obj) = heap.objects.get(&addr) {
        obj.header.marked
    } else {
        false
    }
}

pub fn unmark(heap: &mut Heap, addr: ObjAddr) {
    if let Some(obj) = heap.objects.get_mut(&addr) {
        obj.header.marked = false;
    }
}
