use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    error::VMError,
    free_list::{merge_free_list, FreeList},
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
            free_list: vec![(0, size)],
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

    pub fn free_object(&mut self, addr: ObjAddr) {
        if let Some(obj) = self.objects.remove(&addr) {
            let size = obj.size();
            // Update the free_list to include the memory location previously occupied by the object
            self.free_list.push((addr, addr + size - 1));
        }
    }
    pub fn merge_free_ranges(&mut self) {
        self.free_list = merge_free_list(self.free_list.to_vec());
    }
}
