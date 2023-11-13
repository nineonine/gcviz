use std::collections::HashMap;

use crate::{
    gc::{
        common::move_object,
        object_marker::{is_marked, unmark},
        GCEvent,
    },
    heap::Heap,
    object::{Address, Field},
};

pub fn compact(heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
    let start: usize = 0;
    let end: usize = heap.last_object_addr().unwrap();
    compute_locations(heap, eventlog, start, end, start);
    update_references(heap, eventlog);
    relocate(heap, eventlog, start, end);
}

fn compute_locations(
    heap: &mut Heap,
    eventlog: &mut Vec<GCEvent>,
    start: usize,
    end: usize,
    to_region: usize,
) {
    eventlog.push(GCEvent::phase("compute_locations".to_string()));
    let mut scan = start;
    let mut free = to_region;

    while scan <= end {
        let mut size = 0;

        if is_marked(heap, scan) {
            if let Some(obj) = heap.objects.get_mut(&scan) {
                obj.header.fwd_addr = Some(free);
                size = obj.size();
            }
        }

        if size > 0 {
            free = heap.aligned_position(free + size);
        }

        // Move to the next object
        if let Some(addr) = heap.next_object_addr(scan) {
            scan = addr;
        } else {
            break;
        }
    }
}

fn update_references(heap: &mut Heap, eventlog: &mut Vec<GCEvent>) {
    eventlog.push(GCEvent::phase("update_references".to_string()));
    let mut to_update = HashMap::new();

    // First pass: collect all addresses that need to be updated
    for (addr, obj) in &heap.objects {
        if obj.header.marked {
            for (i, field) in obj.fields.iter().enumerate() {
                if let Field::Ref {
                    addr: Address::Ptr(a),
                } = field
                {
                    let fwd_obj_addr = heap.lookup_object_addr(*a).unwrap();
                    let fwd_addr_w_offset = heap
                        .objects
                        .get(&fwd_obj_addr)
                        .unwrap()
                        .header
                        .fwd_addr
                        .unwrap()
                        + (a - fwd_obj_addr);
                    to_update
                        .entry(*addr)
                        .or_insert_with(Vec::new)
                        .push((i, fwd_addr_w_offset));
                }
            }
        }
    }

    // Second pass: update the addresses
    for (addr, updates) in to_update {
        if let Some(obj) = heap.objects.get_mut(&addr) {
            for (i, fwd_addr) in updates {
                if let Field::Ref {
                    addr: Address::Ptr(ref mut a_ref),
                } = &mut obj.fields[i]
                {
                    if *a_ref == fwd_addr {
                        continue;
                    }
                    eventlog.push(GCEvent::UpdateFwdPtr {
                        old: *a_ref,
                        new: fwd_addr,
                    });
                    *a_ref = fwd_addr;
                }
            }
        }
    }
}

fn relocate(heap: &mut Heap, eventlog: &mut Vec<GCEvent>, start: usize, end: usize) {
    eventlog.push(GCEvent::phase("relocate  ".to_string()));
    let mut scan = start;

    while scan <= end {
        if let Some(obj) = heap.objects.get(&scan) {
            if obj.header.marked {
                if let Some(dest) = obj.header.fwd_addr {
                    if scan != dest {
                        move_object(heap, eventlog, scan, dest);
                    }
                    unmark(heap, dest);
                }
            }
        }

        // Move to the next object
        if let Some(addr) = heap.next_object_addr(scan) {
            scan = addr;
        } else {
            break;
        }
    }
}
