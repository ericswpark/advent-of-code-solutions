#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 14);
        //assert_eq!(part_2(&input), 14);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 327);
        //assert_eq!(part_2(&input), 336495597913098);
    }

    #[test]
    fn simple_test_two_antennas() {
        let input: Vec<String> = String::from(
            "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        )
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect();
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn simple_test_three_antennas() {
        let input: Vec<String> = String::from(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        )
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect();
        assert_eq!(part_1(&input), 4);
    }
}
