use crate::{gc::GCEvent, heap::Heap, object::ObjAddr};

pub fn move_object(heap: &mut Heap, eventlog: &mut Vec<GCEvent>, from: ObjAddr, to: ObjAddr) {
    if heap.objects.get(&from).is_some() {
        let size = heap.objects.get(&from).unwrap().size();
        eventlog.push(GCEvent::MoveObject { from, to, size });
        match heap.move_object(from, to) {
            Ok(res) => res,
            Err(_e) => panic!("move_object {_e}"),
        };
    }
}
