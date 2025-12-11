#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 7);
        // assert_eq!(part_2(&input), 24);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        // assert_eq!(part_1(&input), 4756718172);
        // assert_eq!(part_2(&input), 1665679194);
    }
}
