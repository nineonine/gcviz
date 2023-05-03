use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    error::VMError,
    object::{ObjAddr, Object},
    ui::heap::MemoryCell,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Heap {
    pub roots: HashSet<ObjAddr>,
    pub objects: BTreeMap<ObjAddr, Object>,
    pub free_list: Vec<(usize, usize)>,
    pub memory: Vec<MemoryCell>,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        Heap {
            roots: HashSet::new(),
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

    pub fn free_memory(&self) -> usize {
        self.free_list.iter().map(|(_, size)| size).sum()
    }
}
