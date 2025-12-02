#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 6);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 1007);
        assert_eq!(part_2(&input), 5820);
    }

    #[test]
    fn multiple_rotations_at_once() {
        let input = [String::from("R1000")];
        assert_eq!(part_1(&input), 0);
        assert_eq!(part_2(&input), 10);
    }

    #[test]
    fn rotate_to_zero() {
        let input = [String::from("L50")];
        assert_eq!(part_1(&input), 1);
        assert_eq!(part_2(&input), 1);

        let input = [String::from("L50"), String::from("R100")];
        assert_eq!(part_1(&input), 2);
        assert_eq!(part_2(&input), 2);

        let input = [
            String::from("L50"),
            String::from("R100"),
            String::from("R200"),
        ];
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 4);

        let input = [
            String::from("L50"),
            String::from("R100"),
            String::from("L300"),
        ];
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 5);
    }

    #[test]
    fn boundaries() {
        let input = [String::from("L49")];
        assert_eq!(part_1(&input), 0);
        assert_eq!(part_2(&input), 0);

        let input = [String::from("L49"), String::from("L1")];
        assert_eq!(part_1(&input), 1);
        assert_eq!(part_2(&input), 1);

        let input = [String::from("L49"), String::from("L2")];
        assert_eq!(part_1(&input), 0);
        assert_eq!(part_2(&input), 1);
    }
}
