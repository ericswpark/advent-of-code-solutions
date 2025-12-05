#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 14);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 613);
        assert_eq!(part_2(&input), 336495597913098);
    }

    #[test]
    fn merge_range_check() {
        let ranges = vec![Range { start: 1, end: 5 }, Range { start: 2, end: 10 }];
        let merged_ranges = vec![Range { start: 1, end: 10 }];
        assert_eq!(merge_ranges(&ranges), merged_ranges);
    }

    #[test]
    fn puzzle_check_merge() {
        let input = get_input("puzzle-input.txt");
        let data = parse_input(&input, false);
        let merged_ranges = merge_ranges(&data.ranges);

        for (index, range) in merged_ranges.iter().enumerate() {
            for (oindex, orange) in merged_ranges.iter().enumerate() {
                if index == oindex {
                    continue;
                }

                assert!(
                    !orange.includes(range.start) && !orange.includes(range.end),
                    "{:?} overlaps with {:?}!",
                    range,
                    orange
                );
            }
        }
    }
}
