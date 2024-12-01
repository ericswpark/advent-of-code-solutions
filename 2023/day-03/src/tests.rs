#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use crate::helper_wrappers::get_input;

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input-1.txt");
        assert_eq!(part_1(&input), 4361);
        assert_eq!(part_2(&input), 467835);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 543867);
        assert_eq!(part_2(&input), 79613331);
    }
}
