#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 3749);
        assert_eq!(part_2(&input), 11387);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 42283209483350);
        assert_eq!(part_2(&input), 1026766857276279);
    }
}
