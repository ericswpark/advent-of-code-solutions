#[cfg(test)]
mod tests {
    use crate::*;
    use helpers::get_input;

    #[test]
    fn roll_map_north_test() {
        let input = get_input("sample-input.txt");
        let map = parse_map(&input);
        let rolled_north_map = roll_map_north(map);
        let output = get_input("sample-input-rolled-north.txt");
        let output_map = parse_map(&output);
        assert_eq!(rolled_north_map, output_map);
    }

    #[test]
    fn roll_map_south_test() {
        let input = get_input("sample-input.txt");
        let map = parse_map(&input);
        let rolled_south_map = roll_map_south(map);
        let output = get_input("sample-input-rolled-south.txt");
        let output_map = parse_map(&output);
        assert_eq!(rolled_south_map, output_map);
    }

    #[test]
    fn roll_map_west_test() {
        let input = get_input("sample-input.txt");
        let map = parse_map(&input);
        let rolled_west_map = roll_map_west(map);
        let output = get_input("sample-input-rolled-west.txt");
        let output_map = parse_map(&output);
        assert_eq!(rolled_west_map, output_map);
    }

    #[test]
    fn roll_map_east_test() {
        let input = get_input("sample-input.txt");
        let map = parse_map(&input);
        let rolled_east_map = roll_map_east(map);
        let output = get_input("sample-input-rolled-east.txt");
        let output_map = parse_map(&output);
        assert_eq!(rolled_east_map, output_map);
    }

    #[test]
    fn roll_map_cycle_test() {
        let input = get_input("sample-input.txt");
        let map = parse_map(&input);
        let spinned_map = spin_cycle(map);
        let output = get_input("sample-input-cycle.txt");
        let output_map = parse_map(&output);
        assert_eq!(spinned_map, output_map);
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
        assert_eq!(part_2(&input), 102055);
    }
}
