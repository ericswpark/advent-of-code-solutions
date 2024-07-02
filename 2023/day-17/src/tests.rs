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
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 102);
    }


    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        // assert_eq!(part_1(&input), 8323);
        // assert_eq!(part_2(&input), 8491);
    }
}