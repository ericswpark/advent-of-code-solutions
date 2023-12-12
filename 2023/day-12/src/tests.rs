#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 374);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 9403026);
        assert_eq!(part_2(&input), 543018317006);
    }
}