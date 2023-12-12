#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    #[test]
    fn arrangement_test_1() {
        let row = parse_row(&String::from("???.### 1,1,3"));
        assert_eq!(get_arrangements(row), 1);
    }

    #[test]
    fn arrangement_test_2() {
        let row = parse_row(&String::from(".??..??...?##. 1,1,3"));
        assert_eq!(get_arrangements(row), 4);
    }

    #[test]
    fn arrangement_test_3() {
        let row = parse_row(&String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(get_arrangements(row), 1);
    }

    #[test]
    fn arrangement_test_4() {
        let row = parse_row(&String::from("????.#...#... 4,1,1"));
        assert_eq!(get_arrangements(row), 1);
    }

    #[test]
    fn arrangement_test_5() {
        let row = parse_row(&String::from("????.######..#####. 1,6,5"));
        assert_eq!(get_arrangements(row), 4);
    }

    #[test]
    fn arrangement_test_6() {
        let row = parse_row(&String::from("?###???????? 3,2,1"));
        assert_eq!(get_arrangements(row), 10);
    }

    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 21);
    }


    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        //assert_eq!(part_1(&input), 7975);
        //assert_eq!(part_2(&input), 543018317006);
    }
}