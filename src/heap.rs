use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    error::VMError,
    free_list::FreeList,
    object::{ObjAddr, Object},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Heap {
    pub roots: BTreeSet<ObjAddr>,
    pub objects: BTreeMap<ObjAddr, Object>,
    pub free_list: FreeList,
    pub memory: Vec<MemoryCell>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MemoryCell {
    pub status: CellStatus,
}

impl MemoryCell {
    pub fn new(status: CellStatus) -> Self {
        MemoryCell { status }
    }

    pub fn free() -> Self {
        MemoryCell {
            status: CellStatus::Free,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CellStatus {
    Free,
    Allocated,
    Marked,
    Used,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        Heap {
            roots: BTreeSet::new(),
            objects: BTreeMap::new(),
            memory: vec![MemoryCell::free(); size],
            free_list: free_list![(0, size)],
        }
    }

    /// Attempts to find an `Object` in the heap based on the provided memory `address`.
    ///
    /// # Parameters
    /// - `address`: The memory address to look up.
    ///
    /// # Returns
    /// - `Ok(ObjAddr)`: If an object is found that encompasses the given `address`,
    ///                  the function returns the address at which the object starts in memory.
    /// - `Err(VMError::SegmentationFault)`: If no such object is found, the function returns a segmentation fault error.
    pub fn lookup_object_addr(&self, address: ObjAddr) -> Result<ObjAddr, VMError> {
        // Find the first object that has an address less than or equal to the given address.
        if let Some((obj_addr, object)) = self.objects.range(..=address).next_back() {
            let object_size = object.size();
            if *obj_addr <= address && address < *obj_addr + object_size {
                return Ok(*obj_addr);
            }
        }
        Err(VMError::SegmentationFault)
    }

    pub fn calc_free_memory(&self) -> usize {
        self.free_list.iter().map(|(_, size)| size).sum()
    }

    pub fn merge_free_ranges(&mut self) {
        self.free_list.merge_adjacent_blocks();
    }

    pub fn free_object(&mut self, addr: ObjAddr) -> Result<(), VMError> {
        if let Some(object) = self.objects.remove(&addr) {
            let size = object.size();

            // Add the deallocated space back to free_list
            self.free_list.insert(addr, size);

            // Use unified merge function
            self.free_list.merge_adjacent_blocks();

            // Remove the deallocated object address from the roots set, if present
            self.roots.remove(&addr);
            Ok(())
        } else {
            Err(VMError::DeallocationError) // Error type for failed deallocation
        }
    }

    pub fn move_object(&mut self, from: usize, to: usize) -> Result<(), VMError> {
        // Try to get the object from the `from` address
        let object = if let Some(object) = self.objects.get(&from) {
            object.clone()
        } else {
            return Err(VMError::SegmentationFault);
        };

        // Free the memory at the `from` address
        // this updates roots, objects and free_list
        self.free_object(from)?;

        // Insert the saved object at the new 'to' address
        self.objects.insert(to, object.clone());

        // If the object was a root at its old address, then update the roots set
        if self.roots.contains(&from) {
            self.roots.remove(&from);
            self.roots.insert(to);
        }

        // Update the free_list for the 'to' address by removing the 'to' block
        // and adding a block that accounts for the object's size
        let object_size = object.size();
        self.objects.insert(to, object);
        self.free_list.insert(to, object_size);
        // update free_list to account for new block
        self.free_list.merge_adjacent_blocks();

        Ok(())
    }

    pub fn redraw_memory(&mut self) {
        // Reset all memory cells to Free
        for cell in &mut self.memory {
            cell.status = CellStatus::Free;
        }

        // Set the memory cells occupied by objects to Allocated
        for (addr, object) in &self.objects {
            let size = object.size();
            for offset in 0..size {
                if let Some(cell) = self.memory.get_mut(*addr + offset) {
                    cell.status = CellStatus::Allocated;
                }
            }
        }
    }

    pub fn last_object_addr(&self) -> Option<ObjAddr> {
        self.objects.keys().last().cloned()
    }

    // Returns the address of the object immediately after the given address
    pub fn next_object_addr(&self, addr: ObjAddr) -> Option<ObjAddr> {
        let mut iter = self.objects.keys().clone();
        // Find the provided key in the iterator
        while let Some(next_addr) = iter.next() {
            if *next_addr == addr {
                // Return the next key after the provided key
                return iter.next().cloned();
            }
        }
        None
    }

    // Returns the address of the object immediately before the given address
    pub fn prev_object_addr(&self, addr: ObjAddr) -> Option<ObjAddr> {
        let mut previous = None;
        for key in self.objects.keys() {
            if *key == addr {
                return previous;
            }
            previous = Some(*key);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Field;

    use super::*;

    #[test]
    fn test_new_heap() {
        let heap = Heap::new(100);
        assert_eq!(heap.memory.len(), 100);
        assert_eq!(heap.calc_free_memory(), 100);
        assert!(heap.objects.is_empty());
    }

    #[test]
    fn test_lookup_object_addr() {
        let mut heap = Heap::new(10);
        let obj = Object::new(vec![
            Field::new_scalar(1),
            Field::new_scalar(2),
            Field::new_scalar(3),
        ]);
        heap.objects.insert(0, obj);
        assert_eq!(heap.lookup_object_addr(0), Ok(0));
        assert_eq!(heap.lookup_object_addr(1), Ok(0));
        assert_eq!(heap.lookup_object_addr(2), Ok(0));
        assert_eq!(heap.lookup_object_addr(3), Err(VMError::SegmentationFault));
    }

    #[test]
    fn test_next_object_addr() {
        let mut heap = Heap::new(100);

        // Add objects to the heap.
        let o1 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(0, o1);

        let o2 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(2, o2);

        let o3 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(10, o3);

        // Check next object address for existing objects and some other addresses.
        assert_eq!(heap.next_object_addr(0), Some(2));
        assert_eq!(heap.next_object_addr(2), Some(10));
        assert_eq!(heap.next_object_addr(10), None); // last object, should return None
        assert_eq!(heap.next_object_addr(1), None); // address within the first object
        assert_eq!(heap.next_object_addr(3), None); // address between two objects
        assert_eq!(heap.next_object_addr(11), None); // address within the last object
        assert_eq!(heap.next_object_addr(100), None); // address outside of any object
    }

    #[test]
    fn test_last_object_addr() {
        let mut heap = Heap::new(100);

        // Initially, the heap is empty, so the last object address should be None.
        assert_eq!(heap.last_object_addr(), None);

        // Add an object and check.
        let o1 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(0, o1);
        assert_eq!(heap.last_object_addr(), Some(0));

        // Add another object and check again.
        let o2 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(2, o2);
        assert_eq!(heap.last_object_addr(), Some(2));

        // Add yet another object and check.
        let o3 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(10, o3);
        assert_eq!(heap.last_object_addr(), Some(10));
    }

    #[test]
    fn test_next_prev_object_addr() {
        let mut heap = Heap::new(100);
        // Add multiple objects and navigate between them using next/prev functions.
        let o1 = Object::new(vec![Field::new_scalar(1)]);
        let o2 = Object::new(vec![Field::new_scalar(1)]);
        let o3 = Object::new(vec![Field::new_scalar(1)]);
        heap.objects.insert(0, o1);
        heap.objects.insert(10, o2);
        heap.objects.insert(20, o3);

        // Check the next address of various addresses.
        assert_eq!(heap.next_object_addr(0), Some(10));
        assert_eq!(heap.next_object_addr(10), Some(20));
        assert_eq!(heap.next_object_addr(20), None);

        // Check the previous address of various addresses.
        assert_eq!(heap.prev_object_addr(5), None);
        assert_eq!(heap.prev_object_addr(10), Some(0));
        assert_eq!(heap.prev_object_addr(20), Some(10));

        // Check for wrong addresses.
        assert_eq!(heap.next_object_addr(15), None);
        assert_eq!(heap.prev_object_addr(15), None);
    }
}
