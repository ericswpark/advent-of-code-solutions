mod tests;

use helpers::*;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let map = parse_map(input);
    let rolled_north_map = roll_map_north(map);
    calculate_north_load(&rolled_north_map)
}

fn part_2(input: &Vec<String>) -> i64 {
    let mut map = parse_map(input);

    let mut spinned_maps: Vec<Vec<Vec<Plot>>> = Vec::new();

    let mut i = 0;
    let cycle_count = 1000000000;
    let cycle_frequency;

    'frequency_detection: loop {
        map = spin_cycle(map);

        for (index, spinned_map) in spinned_maps.iter().enumerate() {
            if map == *spinned_map {
                cycle_frequency = (i - index) as i32;
                break 'frequency_detection;
            }
        }

        spinned_maps.push(map.clone());
        i += 1;
    }

    i = cycle_count - ((cycle_count - i) % cycle_frequency as usize);
    i += 1;

    while i < cycle_count {
        map = spin_cycle(map);
        i += 1;
    }

    calculate_north_load(&map)
}

fn spin_cycle(map: Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let north_spin = roll_map_north(map);
    let west_spin = roll_map_west(north_spin);
    let south_spin = roll_map_south(west_spin);
    roll_map_east(south_spin)
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
enum Plot {
    Round,
    Fixed,
    Empty,
}

impl Plot {
    fn mapping(c: char) -> Self {
        match c {
            'O' => Plot::Round,
            '#' => Plot::Fixed,
            '.' => Plot::Empty,
            _ => panic!("Bad plot character!"),
        }
    }
}

fn parse_map(input: &Vec<String>) -> Vec<Vec<Plot>> {
    let mut map = Vec::new();

    for line in input {
        let mut row = Vec::new();

        for plot in line.chars() {
            row.push(Plot::mapping(plot))
        }

        map.push(row)
    }

    map
}

fn roll_map_north(map: Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut map = map;

    for column in 0..map[0].len() {
        let mut start_index = 0;
        let mut round_rocks_encountered = 0;

        // Start from the top and work way down
        for row in 0..map.len() {
            match map[row][column] {
                Plot::Empty => {}
                Plot::Round => {
                    round_rocks_encountered += 1;
                }
                Plot::Fixed => {
                    // Starting from the start index, roll the rocks!
                    for roll_row in map.iter_mut().take(row + 1).skip(start_index) {
                        if round_rocks_encountered > 0 {
                            roll_row[column] = Plot::Round;
                            round_rocks_encountered -= 1;
                        } else if roll_row[column] != Plot::Fixed {
                            roll_row[column] = Plot::Empty;
                        }
                    }
                    // Reset start index to current
                    start_index = row + 1;
                }
            }

            // Check if we're on the last item and we haven't encountered a fixed rock yet
            if row == map.len() - 1 && map[row][column] != Plot::Fixed {
                // Roll the rocks one last time
                for roll_row in map.iter_mut().take(row + 1).skip(start_index) {
                    if round_rocks_encountered > 0 {
                        roll_row[column] = Plot::Round;
                        round_rocks_encountered -= 1;
                    } else {
                        roll_row[column] = Plot::Empty;
                    }
                }
            }
        }
    }

    map
}

fn roll_map_south(map: Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut map = map;

    for column in 0..map[0].len() {
        let mut start_index = map.len() - 1;
        let mut round_rocks_encountered = 0;

        // Start from the bottom and work way up
        for row in (0..map.len()).rev() {
            match map[row][column] {
                Plot::Empty => {}
                Plot::Round => {
                    round_rocks_encountered += 1;
                }
                Plot::Fixed => {
                    // Starting from the start index, roll the rocks!
                    for roll_row in (row..=start_index).rev() {
                        if round_rocks_encountered > 0 {
                            map[roll_row][column] = Plot::Round;
                            round_rocks_encountered -= 1;
                        } else if map[roll_row][column] != Plot::Fixed {
                            map[roll_row][column] = Plot::Empty;
                        }
                    }
                    // Reset start index to current
                    start_index = if row > 0 { row - 1 } else { 0 };
                }
            }

            // Check if we're on the last item and we haven't encountered a fixed rock yet
            if row == 0 && map[row][column] != Plot::Fixed {
                // Roll the rocks one last time
                for roll_row in (row..=start_index).rev() {
                    if round_rocks_encountered > 0 {
                        map[roll_row][column] = Plot::Round;
                        round_rocks_encountered -= 1;
                    } else {
                        map[roll_row][column] = Plot::Empty;
                    }
                }
            }
        }
    }

    map
}

fn roll_map_west(map: Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut map = map;

    for row in 0..map.len() {
        let mut start_index = 0;
        let mut round_rocks_encountered = 0;

        // Start from the left and work way right
        for column in 0..map[0].len() {
            match map[row][column] {
                Plot::Empty => {}
                Plot::Round => {
                    round_rocks_encountered += 1;
                }
                Plot::Fixed => {
                    // Starting from the start index, roll the rocks!
                    for roll_column in start_index..=column {
                        if round_rocks_encountered > 0 {
                            map[row][roll_column] = Plot::Round;
                            round_rocks_encountered -= 1;
                        } else if map[row][roll_column] != Plot::Fixed {
                            map[row][roll_column] = Plot::Empty;
                        }
                    }
                    // Reset start index to current
                    start_index = column + 1;
                }
            }

            // Check if we're on the last item and we haven't encountered a fixed rock yet
            if column == map[row].len() - 1 && map[row][column] != Plot::Fixed {
                // Roll the rocks one last time
                for roll_column in start_index..=column {
                    if round_rocks_encountered > 0 {
                        map[row][roll_column] = Plot::Round;
                        round_rocks_encountered -= 1;
                    } else {
                        map[row][roll_column] = Plot::Empty;
                    }
                }
            }
        }
    }

    map
}

fn roll_map_east(map: Vec<Vec<Plot>>) -> Vec<Vec<Plot>> {
    let mut map = map;

    for row in 0..map.len() {
        let mut start_index = map[0].len() - 1;
        let mut round_rocks_encountered = 0;

        // Start from the right and work way left
        for column in (0..map[0].len()).rev() {
            match map[row][column] {
                Plot::Empty => {}
                Plot::Round => {
                    round_rocks_encountered += 1;
                }
                Plot::Fixed => {
                    // Starting from the start index, roll the rocks!
                    for roll_column in (column..=start_index).rev() {
                        if round_rocks_encountered > 0 {
                            map[row][roll_column] = Plot::Round;
                            round_rocks_encountered -= 1;
                        } else if map[row][roll_column] != Plot::Fixed {
                            map[row][roll_column] = Plot::Empty;
                        }
                    }
                    // Reset start index to current
                    start_index = if column > 0 { column - 1 } else { 0 };
                }
            }

            // Check if we're on the last item and we haven't encountered a fixed rock yet
            if column == 0 && map[row][column] != Plot::Fixed {
                // Roll the rocks one last time
                for roll_column in (column..=start_index).rev() {
                    if round_rocks_encountered > 0 {
                        map[row][roll_column] = Plot::Round;
                        round_rocks_encountered -= 1;
                    } else {
                        map[row][roll_column] = Plot::Empty;
                    }
                }
            }
        }
    }

    map
}

fn calculate_north_load(map: &[Vec<Plot>]) -> i64 {
    let mut load: i64 = 0;
    for row in 0..map.len() {
        let mut rock_count = 0;

        for rock in &map[row] {
            if *rock == Plot::Round {
                rock_count += 1;
            }
        }

        load += (map.len() - row) as i64 * rock_count;
    }

    load
}
