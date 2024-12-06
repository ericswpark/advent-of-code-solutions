#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 41);
        assert_eq!(part_2(&input), 6);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 4939);
        assert_eq!(part_2(&input), 1434);
    }
}
