use crate::{
    error::VMError,
    heap::{Heap, CellStatus},
    object::{ObjAddr, Object},
};

pub struct Allocator {}

impl Allocator {
    pub fn new() -> Self {
        Allocator {}
    }

    pub fn allocate(&self, heap: &mut Heap, object: Object) -> Result<ObjAddr, VMError> {
        // Compute the required size based on the object's fields and alignment
        let size = object.size();

        let free_block_index = heap
            .free_list
            .iter()
            .position(|&(_, block_size)| block_size >= size);

        if let Some(index) = free_block_index {
            let (block_start, block_size) = heap.free_list.remove(index);
            let remaining_size = block_size - size;

            if remaining_size > 0 {
                let new_free_block_start = block_start + size;
                heap.free_list.push((new_free_block_start, remaining_size));
            }

            // Store the object in the memory
            for cell in &mut heap.memory[block_start..block_start + size] {
                cell.status = CellStatus::Allocated;
            }

            // Add the object to the roots
            heap.objects.insert(block_start, object);
            heap.roots.insert(block_start);

            Ok(block_start)
        } else {
            Err(VMError::AllocationError)
        }
    }
}
