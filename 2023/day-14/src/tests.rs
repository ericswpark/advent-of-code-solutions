#[cfg(test)]
mod tests {
    use crate::helpers::get_input;
    use crate::*;

    #[test]
    fn roll_map_north_test() {
        let input_string = String::from("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....");
        let input: Vec<String> = input_string.split("\n").map(|s| s.to_string()).collect();
        let map = parse_map(&input);
        let rolled_north_map = roll_map_north(map);

        let output_string = String::from("OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....");
        let output: Vec<String> = output_string.split("\n").map(|s| s.to_string()).collect();
        let output_map = parse_map(&output);
        assert_eq!(rolled_north_map, output_map);
    }


    #[test]
    fn sample_input() {
        let input = get_input("sample-input.txt");
        assert_eq!(part_1(&input), 136);
        assert_eq!(part_2(&input), 64);
    }


    #[test]
    fn puzzle_input() {
        let input = get_input("puzzle-input.txt");
        assert_eq!(part_1(&input), 111979);
        //assert_eq!(part_2(&input), 27587);
    }
}