#[cfg(test)]
mod tests {
    use helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn part_1_input_1() {
        let input = get_input("part-1-input-1.txt");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_1_input_2() {
        let input = get_input("part-1-input-2.txt");
        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn part_2_input() {
        let input = get_input("part-2-input.txt");
        assert_eq!(part_2(&input), 6);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 16531);
        assert_eq!(part_2(&input), 24035773251517);
    }
}