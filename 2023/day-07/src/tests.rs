#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::part_1;

    #[test]
    fn reddit_sostratus() {
        let input = get_input("reddit-test-case/Sostratus.txt");
        assert_eq!(part_1(&input), 1343);
        //assert_eq!(part_2(&input), 1369);
    }

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 6440);
    }
}