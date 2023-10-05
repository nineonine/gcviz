pub type FreeList = Vec<(usize, usize)>;

pub fn merge_free_list(mut free_list: FreeList) -> FreeList {
    if free_list.is_empty() {
        return free_list.to_vec();
    }

    free_list.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged = Vec::new();
    let mut current = free_list[0];

    for &(start, end) in free_list.iter().skip(1) {
        if current.1 + 1 >= start {
            // They are adjacent or overlapping
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

#[cfg(test)]
mod tests {
    use crate::free_list::merge_free_list;

    #[test]
    fn test_no_merge_needed() {
        let free_list = vec![(1, 3), (5, 7)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 3), (5, 7)]);
    }

    #[test]
    fn test_merge_adjacent_ranges() {
        let free_list = vec![(1, 3), (4, 7)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 7)]);
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        let free_list = vec![(1, 5), (3, 7)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 7)]);
    }

    #[test]
    fn test_merge_multiple_ranges() {
        let free_list = vec![(1, 3), (4, 7), (10, 13), (12, 15)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 7), (10, 15)]);
    }
}
