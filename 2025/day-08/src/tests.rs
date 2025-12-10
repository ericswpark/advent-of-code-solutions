#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(build_and_get_circuit_result(&input, 10), 40);
        assert_eq!(part_2(&input), 25272);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 66912);
        assert_eq!(part_2(&input), 724454082);
    }
}
