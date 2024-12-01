#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use helpers::get_input;

    #[test]
    fn sample_input_1() {
        let input = get_input("sample-input-1.txt");
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn sample_input_2() {
        let input = get_input("sample-input-2.txt");
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn sample_input_3() {
        let input = get_input("sample-input-3.txt");
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn sample_input_4() {
        let input = get_input("sample-input-4.txt");
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn sample_input_5() {
        let input = get_input("sample-input-5.txt");
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn sample_input_6() {
        let input = get_input("sample-input-6.txt");
        assert_eq!(part_1(&input), -1);
    }

    #[test]
    fn sample_input_7() {
        let input = get_input("sample-input-7.txt");
        assert_eq!(part_1(&input), -1);
    }

    #[test]
    fn sample_input_8() {
        let input = get_input("sample-input-8.txt");
        assert_eq!(part_1(&input), -3);
    }

    #[test]
    fn sample_input_9() {
        let input = get_input("sample-input-9.txt");
        assert_eq!(part_1(&input), -3);
    }

    #[test]
    fn sample_input_10() {
        let input = get_input("sample-input-10.txt");
        assert_eq!(part_2(&input), 1);
    }

    #[test]
    fn sample_input_11() {
        let input = get_input("sample-input-11.txt");
        assert_eq!(part_2(&input), 5);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 232);
        assert_eq!(part_2(&input), 1783);
    }
}
