#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 11);
        assert_eq!(part_2(&input), 31);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 2378066);
        assert_eq!(part_2(&input), 18934359);
    }
}
