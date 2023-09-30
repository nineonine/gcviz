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
        let size = object.size();

        if let Some(aligned_start) = self.find_suitable_free_block(heap, size) {
            heap.objects.insert(aligned_start, object);
            heap.roots.insert(aligned_start);
            Ok(aligned_start)
        } else {
            Err(VMError::AllocationError)
        }
    }

    fn find_suitable_free_block(&self, heap: &mut Heap, size: usize) -> Option<ObjAddr> {
        for (block_start, block_size) in &heap.free_list {
            let aligned_start = self.aligned_position(*block_start);
            let block_end = aligned_start + size;

            // Check if the block can accommodate the required size after alignment
            if block_end <= *block_start + *block_size {
                // Update free_list
                self.split_free_block(heap, *block_start, *block_size, aligned_start, block_end);
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
        // Remove the block being split from free_list
        heap.free_list.retain(|&(start, _)| start != block_start);

        // Calculate remaining sizes after allocation
        let remaining_size_before = aligned_start - block_start;
        let remaining_size_after = block_start + block_size - block_end;

        // If there's free space before the allocated block, push it back to free_list
        if remaining_size_before > 0 {
            heap.free_list.push((block_start, remaining_size_before));
        }

        // If there's free space after the allocated block, push it back to free_list
        if remaining_size_after > 0 {
            heap.free_list.push((block_end, remaining_size_after));
        }
    }

    // crazy magic! thanks chatbot
    fn aligned_position(&self, position: usize) -> usize {
        if self.alignment == 0 {
            return position;
        }
        (position + (self.alignment - 1)) & !(self.alignment - 1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::heap::MemoryCell;

    use super::*;

    fn create_heap_with_free_list(free_list: Vec<(ObjAddr, usize)>) -> Heap {
        Heap {
            roots: BTreeSet::new(),
            objects: BTreeMap::new(),
            memory: vec![MemoryCell::free(); 10], // Assuming a size of 10 for simplicity
            free_list: free_list,
        }
    }

    #[test]
    fn test_find_suitable_free_block_with_sufficient_space() {
        let mut heap = create_heap_with_free_list(vec![(0, 4)]);
        let allocator = Allocator { alignment: 2 };

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, Some(0));
        assert_eq!(heap.free_list, vec![(3, 1)]); // 3 remaining cells after allocation
    }

    #[test]
    fn test_find_suitable_free_block_without_sufficient_space() {
        let mut heap = create_heap_with_free_list(vec![(0, 2)]);
        let allocator = Allocator { alignment: 2 };

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, None);
        assert_eq!(heap.free_list, vec![(0, 2)]); // Free list remains unchanged
    }

    #[test]
    fn test_find_suitable_free_block_with_alignment() {
        let mut heap = create_heap_with_free_list(vec![(1, 4)]);
        let allocator = Allocator { alignment: 2 };

        let result = allocator.find_suitable_free_block(&mut heap, 3);
        assert_eq!(result, Some(2)); // Starts at 2 because of alignment
        assert_eq!(heap.free_list, vec![(1, 1)]); // Only 1 cell before the allocated space
    }

    #[test]
    fn test_split_free_block_no_remainder() {
        let mut heap = Heap::new(8);
        heap.free_list = vec![(0, 8)];

        let allocator = Allocator::new(4);
        allocator.split_free_block(&mut heap, 0, 8, 0, 4);

        assert_eq!(heap.free_list, vec![(4, 4)]);
    }

    #[test]
    fn test_split_free_block_remainder_before() {
        let mut heap = Heap::new(10);
        heap.free_list = vec![(0, 8)];

        let allocator = Allocator::new(4);
        allocator.split_free_block(&mut heap, 0, 8, 2, 6);

        assert_eq!(heap.free_list, vec![(0, 2), (6, 2)]);
    }

    #[test]
    fn test_split_free_block_remainder_after() {
        let mut heap = Heap::new(10);
        heap.free_list = vec![(0, 8)];

        let allocator = Allocator::new(4);
        allocator.split_free_block(&mut heap, 0, 8, 0, 6);

        assert_eq!(heap.free_list, vec![(6, 2)]);
    }

    #[test]
    fn test_split_free_block_remainders_both_sides() {
        let mut heap = Heap::new(10);
        heap.free_list = vec![(0, 8)];

        let allocator = Allocator::new(4);
        allocator.split_free_block(&mut heap, 0, 8, 2, 6);

        assert_eq!(heap.free_list, vec![(0, 2), (6, 2)]);
    }

    #[test]
    fn test_split_free_block_no_matching_block() {
        let mut heap = Heap::new(8);
        heap.free_list = vec![(0, 4), (4, 4)];

        let allocator = Allocator::new(4);
        allocator.split_free_block(&mut heap, 8, 4, 8, 12);

        assert_eq!(heap.free_list, vec![(0, 4), (4, 4)]); // Free list remains unchanged.
    }

    #[test]
    fn test_aligned_position() {
        let allocator = Allocator { alignment: 4 };

        assert_eq!(allocator.aligned_position(0), 0);
        assert_eq!(allocator.aligned_position(1), 4);
        assert_eq!(allocator.aligned_position(2), 4);
        assert_eq!(allocator.aligned_position(3), 4);
        assert_eq!(allocator.aligned_position(4), 4);
        assert_eq!(allocator.aligned_position(5), 8);

        let allocator = Allocator { alignment: 8 };

        assert_eq!(allocator.aligned_position(0), 0);
        assert_eq!(allocator.aligned_position(5), 8);
        assert_eq!(allocator.aligned_position(8), 8);
        assert_eq!(allocator.aligned_position(9), 16);
    }

    #[test]
    fn test_no_alignment() {
        let allocator = Allocator { alignment: 0 };

        for i in 0..10 {
            assert_eq!(allocator.aligned_position(i), i);
        }
    }
}
