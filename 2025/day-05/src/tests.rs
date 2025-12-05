#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 3);
        //assert_eq!(part_2(&input), 43);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 613);
        //assert_eq!(part_2(&input), 9122);
    }

    #[test]
    fn merge_range_check() {
        let ranges = vec![Range { start: 1, end: 5 }, Range { start: 2, end: 10 }];
        let merged_ranges = vec![Range { start: 1, end: 10 }];
        assert_eq!(merge_ranges(&ranges), merged_ranges);
    }
}
