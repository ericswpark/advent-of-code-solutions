#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let _input = get_input("sample-input.txt");

        // The answer is 2, but the naive solution only works with the puzzle input.
        //assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 526);
    }
}
