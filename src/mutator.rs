use crate::error::VMError;
use crate::heap::Heap;
use crate::object::{Address, Field, ObjAddr, Value};

pub struct Mutator;

impl Default for Mutator {
    fn default() -> Self {
        Self::new()
    }
}

impl Mutator {
    pub fn new() -> Self {
        Mutator {}
    }

    pub fn read(&self, heap: &Heap, address: ObjAddr) -> Result<Value, VMError> {
        self.read_rec(heap, address)
    }

    fn read_rec(&self, heap: &Heap, address: ObjAddr) -> Result<Value, VMError> {
        let object_addr = heap.lookup_object(address)?;
        let object = heap
            .objects
            .get(&object_addr)
            .ok_or(VMError::SegmentationFault)?;

        let field_index = address - object_addr;
        let field = object
            .fields
            .get(field_index)
            .ok_or(VMError::SegmentationFault)?;

        match field {
            Field::Ref(addr) => match addr {
                Address::Ptr(a) => self.read(heap, *a),
                Address::Null => Err(VMError::NullPointerException(format!(
                    "Attempted to dereference NULL address at 0x{address:X}",
                ))),
            },
            Field::Scalar(value) => Ok(*value),
        }
    }

    pub fn write(&self, heap: &mut Heap, address: ObjAddr, value: Value) -> Result<(), VMError> {
        let object_addr = heap.lookup_object(address)?;
        let object = heap
            .objects
            .get_mut(&object_addr)
            .ok_or(VMError::SegmentationFault)?;

        let field_index = address - object_addr;
        let field = object
            .fields
            .get_mut(field_index)
            .ok_or(VMError::SegmentationFault)?;

        match field {
            Field::Ref(addr) => {
                *addr = Address::Ptr(value);
                Ok(())
            }
            Field::Scalar(val) => {
                *val = value;
                Ok(())
            }
        }
    }
}
