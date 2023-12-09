#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 114);
    }
    //
    // #[test]
    // fn part_1_input_2() {
    //     let input = get_input("part-1-input-2.txt");
    //     assert_eq!(part_1(&input), 6);
    // }
    //
    // #[test]
    // fn part_2_input() {
    //     let input = get_input("part-2-input.txt");
    //     assert_eq!(part_2(&input), 6);
    // }
    //
    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 1992273652);
        //assert_eq!(part_2(&input), 24035773251517);
    }
}