#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::{part_1, part_2};

    #[test]
    fn reddit_sostratus() {
        let input = get_input("reddit-test-case/Sostratus.txt");
        assert_eq!(part_1(&input), 1343);
        assert_eq!(part_2(&input), 1369);
    }

    #[test]
    fn reddit_lxstergames() {
        let input = get_input("reddit-test-case/LxsterGames.txt");
        assert_eq!(part_1(&input), 6592);
        assert_eq!(part_2(&input), 6839);
    }

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 6440);
        assert_eq!(part_2(&input), 5905);
    }

    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 248105065);
        assert_eq!(part_2(&input), 249515436);
    }
}