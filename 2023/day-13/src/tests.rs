#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    #[test]
    fn simple_mirror_1() {
        let input = &String::from("##").split("\n\n").map(|s| s.to_string()).collect();
        let patterns = parse_patterns(input);
        assert_eq!(get_vertical_reflection(&patterns[0]), 1);
    }

    #[test]
    fn simple_mirror_2() {
        let input = &String::from("#..#").split("\n\n").map(|s| s.to_string()).collect();
        let patterns = parse_patterns(input);
        assert_eq!(get_vertical_reflection(&patterns[0]), 2);
        assert_eq!(get_horizontal_reflection(&patterns[0]), -1);
    }

    #[test]
    fn simple_mirror_3() {
        let input = &String::from("#.##.#").split("\n\n").map(|s| s.to_string()).collect();
        let patterns = parse_patterns(input);
        assert_eq!(get_vertical_reflection(&patterns[0]), 3);
    }



    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 405);
    }


    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 42974);
        //assert_eq!(part_2(&input), 65607131946466);
    }
}