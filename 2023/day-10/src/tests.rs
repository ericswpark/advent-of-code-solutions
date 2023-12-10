#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn sample_complex_loop() {
        let input = get_input("sample-complex-loop.txt");
        assert_eq!(part_1(&input), 8);
    }

    #[test]
    fn sample_simple_loop() {
        let input = get_input("sample-simple-loop.txt");
        assert_eq!(part_1(&input), 4);
    }

    #[test]
    fn sample_squeeze_pipe() {
        let input = get_input("sample-squeeze-pipe.txt");
        assert_eq!(part_2(&input), 4);
    }

    #[test]
    fn sample_loop_complex_check() {
        let input = get_input("sample-loop-complex-check.txt");
        assert_eq!(part_2(&input), 10);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 6820);
        //assert_eq!(part_2(&input), 0);
    }
}