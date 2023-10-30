use std::collections::HashMap;

use crate::{
    gc::{
        object_marker::{is_marked, unmark},
        GCEvent,
    },
    heap::Heap,
    object::{Address, Field, ObjAddr},
};

// Two-Finger Algorithm,
// introduced by W. L. Edwards in 1974
pub fn compact(heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
    let start: usize = 0;
    let end: usize = heap.last_object_addr().unwrap();
    eventlog.push(GCEvent::phase(format!(
        "Run 2 finger compaction. Start: {start} End: {end}"
    )));
    let forwarding_ptrs = relocate(heap, eventlog, start, end);
    update_references(heap, &forwarding_ptrs, eventlog);
}

fn relocate(
    heap: &mut Heap,
    eventlog: &mut Vec<GCEvent>,
    start: ObjAddr,
    end: ObjAddr,
) -> HashMap<usize, usize> {
    eventlog.push(GCEvent::phase("relocate".to_string()));
    let mut free = start;
    let mut scan = end;

    let mut forwarding_pointers = HashMap::new();

    while free < scan {
        while is_marked(heap, free) {
            unmark(heap, free);
            free = heap.next_object_addr(free).unwrap();
        }

        while !is_marked(heap, scan) && scan > free {
            scan = heap.prev_object_addr(scan).unwrap();
        }

        // Note, that this only works well for regions with allocated objects of same size.
        // The books says: "Note that the quality of compaction depends on the size of the
        // gap at free closely matching the size of the live object at scan. Unless this
        // algorithm is used on fixed-size objects, the degree of defragmentation might
        // be very poor indeed."
        if scan > free && can_fit_into(heap, scan, free) {
            unmark(heap, scan);
            save_forward_ptrs(heap, &mut forwarding_pointers, scan, free);
            move_object(heap, eventlog, scan, free);
            if let Some(next_free) = heap.next_object_addr(free) {
                free = next_free;
            } else {
                break;
            }
            if let Some(prev_scan) = heap.prev_object_addr(scan) {
                scan = prev_scan;
            } else {
                break;
            }
        }
    }

    forwarding_pointers
}

fn save_forward_ptrs(
    heap: &Heap,
    forwarding_pointers: &mut HashMap<usize, usize>,
    from: ObjAddr,
    to: ObjAddr,
) {
    let obj = heap.objects.get(&from).unwrap();

    // Iterate over the fields of the object
    for (offset_from_start, field) in obj.fields.iter().enumerate() {
        match field {
            Field::Ref {
                addr: Address::Ptr(_),
            } => {
                let old_ptr = from + offset_from_start;
                let new_ptr = to + offset_from_start;
                forwarding_pointers.insert(old_ptr, new_ptr);
            }
            Field::Scalar { .. } => {
                let old_ptr = from + offset_from_start;
                let new_ptr = to + offset_from_start;
                forwarding_pointers.insert(old_ptr, new_ptr);
            }
            _ => {}
        }
    }
}

fn can_fit_into(heap: &Heap, move_in: ObjAddr, dest: ObjAddr) -> bool {
    let obj_to_be_moved = heap.objects.get(&move_in).unwrap();
    let obj_to_be_removed = heap.objects.get(&dest).unwrap();
    obj_to_be_moved.size() <= obj_to_be_removed.size()
}

fn move_object(heap: &mut Heap, eventlog: &mut Vec<GCEvent>, from: ObjAddr, to: ObjAddr) {
    if heap.objects.get(&from).is_some() {
        eventlog.push(GCEvent::phase(format!("Moving object from {from} to {to}")));
        match heap.move_object(from, to) {
            Ok(res) => res,
            Err(_e) => panic!("move_object {_e}"),
        };
    }
}

fn update_references(
    heap: &mut Heap,
    forwarding_pointers: &HashMap<usize, usize>,
    eventlog: &mut Vec<GCEvent>,
) {
    for object in heap.objects.values_mut() {
        for field in &mut object.fields {
            if let Field::Ref {
                addr: Address::Ptr(old),
            } = field
            {
                if let Some(new_addr) = forwarding_pointers.get(old) {
                    *old = *new_addr;
                    eventlog.push(GCEvent::UpdateFwdPtr {
                        old: *old,
                        new: *new_addr,
                    });
                }
            }
        }
    }
}
