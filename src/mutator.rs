use crate::error::VMError;
use crate::heap::Heap;
use crate::object::{Value, ObjAddr, Field, Address};

pub struct Mutator;

impl Mutator {
    pub fn new() -> Self {
        Mutator {}
    }

    pub fn read(&self, heap: &Heap, address: ObjAddr) -> Result<Value, VMError> {
        let object_addr = heap.lookup_object(address)?;
        let object = heap.objects.get(&object_addr).ok_or(VMError::SegmentationFault)?;

        let field_index = address - object_addr;
        let field = object.fields.get(field_index).ok_or(VMError::SegmentationFault)?;

        match field {
            Field::Ref(addr) => match addr {
                Address::Ptr(a) => self.read(heap, *a),
                Address::Null => Err(VMError::NullPointerException)
            }
            Field::Scalar(value) => Ok(*value),
        }
    }

pub fn write(
    &self,
    heap: &mut Heap,
    address: ObjAddr,
    value: Value,
) -> Result<(), VMError> {
    let object_addr = heap.lookup_object(address)?;
    let object = heap.objects.get_mut(&object_addr).ok_or(VMError::SegmentationFault)?;

    let field_index = address - object_addr;
    let field = object.fields.get_mut(field_index).ok_or(VMError::SegmentationFault)?;

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
