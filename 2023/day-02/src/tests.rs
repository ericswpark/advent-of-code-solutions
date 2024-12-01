#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use helpers::get_input;

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input-1.txt");
        assert_eq!(part_1(&input), 8);
        assert_eq!(part_2(&input), 2286);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 2913);
        assert_eq!(part_2(&input), 55593);
    }
}
