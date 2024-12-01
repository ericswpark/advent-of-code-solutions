#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use crate::custom_helper::get_input;

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 35);
        assert_eq!(part_2(&input), 46);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 265018614);
        assert_eq!(part_2(&input), 63179500);
    }
}
