use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct FreeList {
    pub inner: BTreeMap<usize, usize>,
}

pub struct FreeListIter<'a> {
    inner_iter: std::collections::btree_map::Iter<'a, usize, usize>,
}

impl<'a> Iterator for FreeListIter<'a> {
    type Item = (&'a usize, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}

impl Serialize for FreeList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the inner BTreeMap to a Vec representation
        let vec_representation = self.to_vec();
        // Serialize the Vec representation
        vec_representation.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FreeList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize into a Vec<(usize, usize)>
        let vec_representation: Vec<(usize, usize)> = Vec::deserialize(deserializer)?;
        // Use the Vec representation to construct a FreeList
        Ok(FreeList::new(vec_representation))
    }
}

impl FreeList {
    /// Constructs a new FreeList from a Vec of (start, size).
    pub fn new(blocks: Vec<(usize, usize)>) -> Self {
        let inner = blocks.into_iter().collect::<BTreeMap<_, _>>();
        Self { inner }
    }

    pub fn iter(&self) -> FreeListIter {
        FreeListIter {
            inner_iter: self.inner.iter(),
        }
    }

    /// Inserts a new free block.
    pub fn insert(&mut self, start: usize, size: usize) {
        if let Some(&len) = self.inner.get(&start) {
            self.inner.insert(start, usize::max(size, len));
        } else {
            self.inner.insert(start, size);
        }
        self.merge_adjacent_blocks();
    }

    /// Removes a free block by its start address.
    pub fn remove(&mut self, start: usize) {
        self.inner.remove(&start);
    }

    /// Merges adjacent blocks in the FreeList.
    pub fn merge_adjacent_blocks(&mut self) {
        let mut current = self.inner.keys().cloned().next();

        while let Some(start1) = current {
            let len1 = *self.inner.get(&start1).unwrap();
            let end1 = start1 + len1;

            if let Some((&start2, &len2)) = self.inner.range((start1 + 1)..).next() {
                if start2 <= end1 {
                    // Overlapping or adjacent blocks found
                    // Update the length of the current block
                    *self.inner.get_mut(&start1).unwrap() = len1 + len2 + start2 - end1;
                    // Remove the next block
                    self.inner.remove(&start2);
                } else {
                    current = Some(start2);
                }
            } else {
                break;
            }
        }
    }

    /// Converts the FreeList to a Vec representation.
    pub fn to_vec(&self) -> Vec<(usize, usize)> {
        self.inner.clone().into_iter().collect()
    }
}

macro_rules! free_list {
    ($(($key:expr, $value:expr)),+) => {
        FreeList {
            inner: {
                let mut map = BTreeMap::new();
                $(map.insert($key, $value);)+
                map
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::FreeList;

    #[test]
    fn test_no_merge_needed() {
        let mut free_list = FreeList::new(vec![(1, 3), (5, 7)]);
        free_list.merge_adjacent_blocks();

        assert_eq!(free_list.to_vec(), vec![(1, 3), (5, 7)]);
    }

    #[test]
    fn test_merge_adjacent_ranges() {
        let mut free_list = FreeList::new(vec![(1, 3), (4, 7)]);
        free_list.merge_adjacent_blocks();

        assert_eq!(free_list.to_vec(), vec![(1, 10)]);
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        let mut free_list = FreeList::new(vec![(1, 5), (3, 7)]);
        free_list.merge_adjacent_blocks();

        assert_eq!(free_list.to_vec(), vec![(1, 9)]);
    }

    #[test]
    fn test_merge_multiple_ranges_collapse_all() {
        let mut free_list = FreeList::new(vec![(1, 3), (4, 7), (10, 13), (12, 15)]);
        free_list.merge_adjacent_blocks();

        assert_eq!(free_list.to_vec(), vec![(1, 26)]);
    }

    #[test]
    fn test_merge_multiple_ranges_two_left() {
        let mut free_list = FreeList::new(vec![(1, 3), (4, 7), (12, 5), (17, 5)]);
        free_list.merge_adjacent_blocks();

        assert_eq!(free_list.to_vec(), vec![(1, 10), (12, 10)]);
    }

    #[test]
    fn test_single_block_no_merge() {
        let mut free_list = FreeList::new(vec![(1, 5)]);
        free_list.merge_adjacent_blocks();
        assert_eq!(free_list.to_vec(), vec![(1, 5)]);
    }

    #[test]
    fn test_merge_multiple_tiny_blocks() {
        let mut free_list = FreeList::new(vec![(1, 1), (2, 1), (3, 1), (5, 1), (6, 1)]);
        free_list.merge_adjacent_blocks();
        assert_eq!(free_list.to_vec(), vec![(1, 3), (5, 2)]);
    }

    #[test]
    fn test_remove_middle_block() {
        let mut free_list = FreeList::new(vec![(1, 5), (6, 5), (11, 5)]);
        free_list.remove(6);
        assert_eq!(free_list.to_vec(), vec![(1, 5), (11, 5)]);
    }

    #[test]
    fn test_insert_block_in_between() {
        let mut free_list = FreeList::new(vec![(1, 5), (11, 5)]);
        free_list.insert(6, 5);
        assert_eq!(free_list.to_vec(), vec![(1, 15)]);
    }

    #[test]
    fn test_adjacent_single_unit_blocks() {
        let mut free_list = FreeList::new(vec![(1, 1), (2, 1), (4, 1)]);
        free_list.merge_adjacent_blocks();
        assert_eq!(free_list.to_vec(), vec![(1, 2), (4, 1)]);
    }

    #[test]
    fn test_large_input() {
        let blocks: Vec<(usize, usize)> = (0..1000).map(|x| (x, 1)).collect();
        let mut free_list = FreeList::new(blocks);
        free_list.merge_adjacent_blocks();
        assert_eq!(free_list.to_vec(), vec![(0, 1000)]);
    }

    #[test]
    fn test_insert() {
        let mut free_list = FreeList::new(vec![(8, 8)]);
        free_list.insert(8, 4);
        assert_eq!(free_list.inner.len(), 1);
        assert_eq!(free_list.inner.get(&8), Some(&8));
        free_list.insert(8, 16);
        assert_eq!(free_list.inner.len(), 1);
        assert_eq!(free_list.inner.get(&8), Some(&16));
        free_list.insert(24, 2);
        assert_eq!(free_list.inner.len(), 1);
        assert_eq!(free_list.inner.get(&8), Some(&18));
        free_list.insert(28, 2);
        assert_eq!(free_list.inner.len(), 2);
        assert_eq!(free_list.inner.get(&8), Some(&18));
        assert_eq!(free_list.inner.get(&28), Some(&2));
    }
}
