#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input-1.txt");
        assert_eq!(part_1(&input), 2);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn sample_input_2() {
        let input = get_input("sample-input-2.txt");
        assert_eq!(part_1(&input), 6);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 16531);
        // assert_eq!(part_2(&input),);
    }
}