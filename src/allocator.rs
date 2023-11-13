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

    pub fn allocate(
        &self,
        heap: &mut Heap,
        object: Object,
        is_root: bool,
    ) -> Result<ObjAddr, VMError> {
        let size = object.size();

        if let Some(aligned_start) = self.find_suitable_free_block(heap, size) {
            heap.objects.insert(aligned_start, object);
            if is_root {
                heap.roots.insert(aligned_start);
            }
            Ok(aligned_start)
        } else {
            Err(VMError::AllocationError)
        }
    }

    fn find_suitable_free_block(&self, heap: &mut Heap, size: usize) -> Option<ObjAddr> {
        // Iterate over the blocks in the FreeList
        for (block_start, block_size) in heap.free_list.to_vec() {
            let aligned_start = heap.aligned_position(block_start);
            let block_end = aligned_start + size;

            // Check if the block can accommodate the required size after alignment
            if block_end <= block_start + block_size {
                // Update free_list
                self.split_free_block(heap, block_start, block_size, aligned_start, block_end);
                return Some(aligned_start);
            }
        }
        None
    }

    fn split_free_block(
        &self,
        heap: &mut Heap,
        block_start: ObjAddr,
        block_size: usize,
        aligned_start: ObjAddr,
        block_end: ObjAddr,
    ) {
        // Remove the block being split from the FreeList
        heap.free_list.remove(block_start);

        // Calculate remaining sizes after allocation
        let remaining_size_before = aligned_start - block_start;
        let remaining_size_after = block_start + block_size - block_end;

        // If there's free space before the allocated block, insert it back into the FreeList
        if remaining_size_before > 0 {
            heap.free_list.insert(block_start, remaining_size_before);
        }

        // If there's free space after the allocated block, insert it back into the FreeList
        if remaining_size_after > 0 {
            heap.free_list.insert(block_end, remaining_size_after);
        }
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::{free_list::FreeList, heap::MemoryCell, object::Field};

    use super::*;

    fn create_heap_with_free_list(alignment: usize, free_list: Vec<(ObjAddr, usize)>) -> Heap {
        Heap {
            roots: BTreeSet::new(),
            objects: BTreeMap::new(),
            memory: vec![MemoryCell::free(); 10], // Assuming a size of 10 for simplicity
            free_list: FreeList::new(free_list),
            alignment,
        }
    }

    #[test]
    fn test_find_suitable_free_block_with_sufficient_space() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 4)]);
        let allocator = Allocator {};

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, Some(0));
        assert_eq!(heap.free_list.to_vec(), vec![(3, 1)]);
    }

    #[test]
    fn test_find_suitable_free_block_first_block() {
        let mut heap = create_heap_with_free_list(2, vec![(2, 2), (8, 2)]);
        let allocator = Allocator {};

        let result = allocator.find_suitable_free_block(&mut heap, 2);
        assert_eq!(result, Some(2));
        assert_eq!(heap.free_list.to_vec(), vec![(8, 2)]);
    }

    #[test]
    fn test_find_suitable_free_block_without_sufficient_space() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 2)]);
        let allocator = Allocator {};

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, None);
        assert_eq!(heap.free_list.to_vec(), vec![(0, 2)]); // Free list remains unchanged
    }

    #[test]
    fn test_find_suitable_free_block_with_alignment() {
        let mut heap = create_heap_with_free_list(2, vec![(1, 4)]);
        let allocator = Allocator {};

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, Some(2)); // Starts at 2 because of alignment
        assert_eq!(heap.free_list.to_vec(), vec![(1, 1)]); // Only 1 cell before the allocated space
    }

    #[test]
    fn test_split_free_block_no_remainder() {
        let mut heap = Heap::new(8, 0);
        heap.free_list = free_list![(0, 8)];

        let allocator = Allocator::new();
        allocator.split_free_block(&mut heap, 0, 8, 0, 4);

        assert_eq!(heap.free_list.to_vec(), vec![(4, 4)]);
    }

    #[test]
    fn test_split_free_block_remainder_before() {
        let mut heap = Heap::new(10, 4);
        heap.free_list = free_list![(0, 8)];

        let allocator = Allocator::new();
        allocator.split_free_block(&mut heap, 0, 8, 2, 6);

        assert_eq!(heap.free_list.to_vec(), vec![(0, 2), (6, 2)]);
    }

    #[test]
    fn test_split_free_block_remainder_after() {
        let mut heap = Heap::new(10, 4);
        heap.free_list = free_list![(0, 8)];

        let allocator = Allocator::new();
        allocator.split_free_block(&mut heap, 0, 8, 0, 6);

        assert_eq!(heap.free_list.to_vec(), vec![(6, 2)]);
    }

    #[test]
    fn test_split_free_block_remainders_both_sides() {
        let mut heap = Heap::new(10, 4);
        heap.free_list = free_list![(0, 8)];

        let allocator = Allocator::new();
        allocator.split_free_block(&mut heap, 0, 8, 2, 6);

        assert_eq!(heap.free_list.to_vec(), vec![(0, 2), (6, 2)]);
    }

    #[test]
    fn test_split_free_block_no_matching_block() {
        let mut heap = Heap::new(8, 4);
        heap.free_list = free_list![(0, 4), (4, 4)];

        let allocator = Allocator::new();
        allocator.split_free_block(&mut heap, 8, 4, 8, 12);

        assert_eq!(heap.free_list.to_vec(), vec![(0, 4), (4, 4)]); // Free list remains unchanged.
    }

    #[test]
    fn test_allocate_with_sufficient_space() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 4)]);
        let allocator = Allocator {};
        let object = Object::new(vec![
            Field::new_scalar(1),
            Field::new_scalar(2),
            Field::new_scalar(3),
        ]);

        let result = allocator.allocate(&mut heap, object, true);
        assert!(result.is_ok());
        assert_eq!(heap.objects.len(), 1); // The object should be added to `heap.objects`.
        assert_eq!(heap.roots.len(), 1); // Since it's marked as root, it should be added to `heap.roots`.
        assert_eq!(heap.free_list.to_vec(), vec![(3, 1)]); // 1 remaining cell after allocation.
    }

    #[test]
    fn test_allocate_without_sufficient_space() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 2)]);
        let allocator = Allocator {};
        let object = Object::new(vec![
            Field::new_scalar(1),
            Field::new_scalar(2),
            Field::new_scalar(3),
        ]);

        let result = allocator.allocate(&mut heap, object, true);
        assert!(result.is_err()); // Allocation should fail.
        assert_eq!(heap.objects.len(), 0); // No object should be added.
        assert_eq!(heap.free_list.to_vec(), vec![(0, 2)]); // Free list should remain unchanged.
    }

    #[test]
    fn test_allocate_multiple_objects() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 10)]);
        let allocator = Allocator {};

        let object1 = Object::new(vec![Field::new_scalar(1), Field::new_scalar(2)]);
        let object2 = Object::new(vec![
            Field::new_scalar(3),
            Field::new_scalar(4),
            Field::new_scalar(5),
        ]);
        let object3 = Object::new(vec![Field::new_scalar(6), Field::new_scalar(7)]);

        allocator.allocate(&mut heap, object1, true).unwrap();
        allocator.allocate(&mut heap, object2, true).unwrap();
        allocator.allocate(&mut heap, object3, false).unwrap();

        assert_eq!(heap.objects.len(), 3); // Three objects should be added.
        assert_eq!(heap.roots.len(), 2); // Only two objects were marked as roots.
                                         // Free list should have remaining spaces depending on the sizes and alignment of the objects.
    }

    #[test]
    fn test_allocate_after_deallocate() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 10)]);
        let allocator = Allocator {};

        let object1 = Object::new(vec![Field::new_scalar(1), Field::new_scalar(2)]);
        let addr1 = allocator.allocate(&mut heap, object1, true).unwrap();

        heap.free_object(addr1).unwrap();

        let object2 = Object::new(vec![Field::new_scalar(3), Field::new_scalar(4)]);
        let addr2 = allocator.allocate(&mut heap, object2, false).unwrap();

        assert_eq!(addr1, addr2); // Should allocate in the same spot as the deallocated object.
        assert_eq!(heap.objects.len(), 1);
        assert_eq!(heap.roots.len(), 0);
    }

    #[test]
    fn test_calc_memory_after_allocate() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 10)]);
        let allocator = Allocator {};

        assert_eq!(heap.calc_free_memory(), 10);

        let object1 = Object::new(vec![Field::new_scalar(1), Field::new_scalar(2)]);
        let addr1 = allocator.allocate(&mut heap, object1, true).unwrap();

        assert_eq!(heap.calc_free_memory(), 8);

        let res = heap.free_object(addr1);
        assert_eq!(res, Ok(()));
        assert_eq!(heap.calc_free_memory(), 10);
    }

    #[test]
    fn test_free_object() {
        let mut heap = create_heap_with_free_list(2, vec![(0, 10)]);
        let allocator = Allocator {};
        let object = Object::new(vec![
            Field::new_scalar(1),
            Field::new_scalar(2),
            Field::new_scalar(3),
        ]);

        let result = allocator.allocate(&mut heap, object, true);
        assert_eq!(result, Ok(0));
        assert_eq!(heap.calc_free_memory(), 7);
        assert_eq!(heap.objects.len(), 1);
        assert_eq!(heap.roots.len(), 1);

        let res = heap.free_object(result.unwrap());
        assert_eq!(res, Ok(()));
        assert_eq!(heap.calc_free_memory(), 10);
        assert!(heap.objects.is_empty());
        assert!(heap.roots.is_empty());
        assert_eq!(heap.free_list.inner.len(), 1);
    }
}
