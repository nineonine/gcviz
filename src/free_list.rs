pub type FreeList = Vec<(usize, usize)>;

pub fn merge_free_list(mut free_list: FreeList) -> FreeList {
    if free_list.is_empty() {
        return free_list;
    }

    free_list.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged = Vec::new();
    let mut current_start = free_list[0].0;
    let mut current_end = current_start + free_list[0].1;

    for &(start, size) in free_list.iter().skip(1) {
        if current_end >= start {
            // Overlapping or adjacent ranges
            current_end = current_end.max(start + size); // Take the maximum end point
        } else {
            merged.push((current_start, current_end - current_start));
            current_start = start;
            current_end = start + size;
        }
    }

    merged.push((current_start, current_end - current_start));
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

        assert_eq!(merged, vec![(1, 10)]);
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        let free_list = vec![(1, 5), (3, 7)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 9)]);
    }

    #[test]
    fn test_merge_multiple_ranges_collapse_all() {
        let free_list = vec![(1, 3), (4, 7), (10, 13), (12, 15)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 26)]);
    }

    #[test]
    fn test_merge_multiple_ranges_two_left() {
        let free_list = vec![(1, 3), (4, 7), (12, 5), (17, 5)];
        let merged = merge_free_list(free_list);

        assert_eq!(merged, vec![(1, 10), (12, 10)]);
    }
}
