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
    ToBeFree,
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

    pub fn lookup_object(&self, address: ObjAddr) -> Result<ObjAddr, VMError> {
        let mut iter = self.objects.range(..=address).rev(); // Get an iterator in reverse order up to the given address
        if let Some((obj_addr, object)) = iter.next() {
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
}
