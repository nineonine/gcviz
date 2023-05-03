use crate::{
    error::VMError,
    heap::Heap,
    object::{ObjAddr, Object},
};

pub struct Allocator {
    pub alignment: usize,
}

impl Allocator {
    pub fn new(alignment: usize) -> Self {
        Allocator { alignment }
    }

    pub fn allocate(&self, heap: &mut Heap, object: Object) -> Result<ObjAddr, VMError> {
        // Compute the required size based on the object's fields and alignment
        let size = object.size();

        let free_block_index =
            heap.free_list
                .iter()
                .enumerate()
                .find_map(|(index, &(block_start, block_size))| {
                    let aligned_start = self.aligned_position(block_start);
                    let aligned_end = aligned_start + size;

                    if aligned_end <= block_start + block_size {
                        Some((index, block_start, aligned_start, aligned_end))
                    } else {
                        None
                    }
                });

        if let Some((index, block_start, aligned_start, aligned_end)) = free_block_index {
            let (_, block_size) = heap.free_list.remove(index);
            let remaining_size_before = aligned_start - block_start;
            let remaining_size_after = block_start + block_size - aligned_end;

            if remaining_size_before > 0 {
                heap.free_list.push((block_start, remaining_size_before));
            }

            if remaining_size_after > 0 {
                heap.free_list.push((aligned_end, remaining_size_after));
            }

            // Store the object
            heap.objects.insert(aligned_start, object);

            // Add the object to the roots
            heap.roots.insert(aligned_start);

            Ok(aligned_start)
        } else {
            Err(VMError::AllocationError)
        }
    }

    fn aligned_position(&self, position: usize) -> usize {
        if self.alignment == 0 {
            return position;
        }

        let remainder = position % self.alignment;
        if remainder == 0 {
            position
        } else {
            position + self.alignment - remainder
        }
    }
}
