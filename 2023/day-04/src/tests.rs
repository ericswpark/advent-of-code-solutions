#[cfg(test)]
mod tests {
    use helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 30);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 23678);
        assert_eq!(part_2(&input), 15455663);
    }
}
