#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn reddit_1234abcdcba4321() {
        let input = get_input("reddit-test-cases/1234abcdcba4321.txt");
        assert_eq!(part_1(&input), 12);
    }

    #[test]
    fn reddit_leftylink() {
        let input = get_input("reddit-test-cases/leftylink.txt");
        assert_eq!(part_1(&input), 62);
        assert_eq!(part_2(&input), 18);
    }

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 102);
        assert_eq!(part_2(&input), 94);
    }

    #[test]
    fn sample_input_2() {
        let input = get_input("sample-input-2.txt");
        assert_eq!(part_1(&input), 59);
        assert_eq!(part_2(&input), 71);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 1110);
        assert_eq!(part_2(&input), 1298);
    }
}
