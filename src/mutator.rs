use crate::error::VMError;
use crate::heap::Heap;
use crate::object::{Address, Offset, Value};

struct Mutator;

impl Mutator {
    pub fn new() -> Self {
        Mutator {}
    }

    pub fn read(&self, heap: Heap, address: Address, offset: Offset) -> Result<Value, VMError> {
        Ok(0)
    }

    pub fn write(
        &self,
        heap: Heap,
        address: Address,
        offset: Offset,
        value: Value,
    ) -> Result<(), VMError> {
        Ok(())
    }
}
