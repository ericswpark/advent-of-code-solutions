#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input-1.txt");
        assert_eq!(part_1(&input), 58);
        assert_eq!(part_2(&input), 34);
    }

    #[test]
    fn sample_input_2() {
        let input = get_input("sample-input-2.txt");
        assert_eq!(part_1(&input), 43);
        assert_eq!(part_2(&input), 14);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 1598415);
        assert_eq!(part_2(&input), 3812909);
    }
}
