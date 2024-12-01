#[cfg(test)]
mod tests {
    use custom_helper::get_input;
    use crate::*;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 1320);
        assert_eq!(part_2(&input), 145);
    }


    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 501680);
        assert_eq!(part_2(&input), 241094);
    }
}