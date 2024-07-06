#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 62);
        // assert_eq!(part_2(&input), 94);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 1110);
        // assert_eq!(part_2(&input), 1294);
    }
}
