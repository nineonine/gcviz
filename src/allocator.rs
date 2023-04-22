use crate::{
    error::VMError,
    heap::Heap,
    object::{ObjAddr, Object},
};

pub struct Allocator {}

impl Allocator {
    pub fn new() -> Self {
        Allocator {}
    }

    pub fn allocate(&self, heap: Heap, object: Object) -> Result<ObjAddr, VMError> {
        Ok(0)
    }
}
