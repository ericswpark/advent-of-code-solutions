#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        //assert_eq!(part_1(&input), 2);
        //assert_eq!(part_2(&input), 2);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 526);
        //assert_eq!(part_2(&input), 362956369749210);
    }
}
