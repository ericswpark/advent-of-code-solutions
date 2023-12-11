mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}


fn part_1(input: &Vec<String>) -> i32 {
    let (galaxy_positions, map_size) = parse_map(input);
    let empty_map = find_empties(&galaxy_positions, map_size);
    let galaxy_positions = find_new_galaxy_positions(&galaxy_positions, empty_map);

    let mut distance_sum = 0;

    for start_galaxy_index in 0..galaxy_positions.len() {
        for end_galaxy_index in start_galaxy_index + 1..galaxy_positions.len() {
            // Find distance between the two galaxies
            distance_sum += get_distance(&galaxy_positions, start_galaxy_index, end_galaxy_index);
        }
    }

    distance_sum
}



fn part_2(_input: &Vec<String>) -> i32 {
    todo!("Implement")
}

struct Position {
    row: i32,
    column: i32,
}

/// Parses map and returns the positions of galaxies, and a Position struct containing the total map
/// size
fn parse_map(input: &Vec<String>) -> (Vec<Position>, Position) {
    let mut positions = Vec::new();

    let mut max_col = 0;
    let mut row = 0;
    for line in input {
        let mut col = 0;
        for char in line.chars() {
            if char == '#' {
                // Found a new galaxy
                positions.push(Position{ row, column: col })
            }

            col += 1;
        }
        // We assume that all rows have the same number of columns
        max_col = col;

        row += 1;
    }

    (positions, Position { row, column: max_col })
}

struct EmptyMap {
    row_empties: Vec<bool>,
    col_empties: Vec<bool>,
}

/// Find empty spaces between galaxies
fn find_empties(galaxy_positions: &Vec<Position>, map_size: Position) -> EmptyMap {
    let mut row_empties = vec![true; map_size.row as usize];
    let mut col_empties = vec![true; map_size.column as usize];

    for position in galaxy_positions {
        row_empties[position.row as usize] = false;
        col_empties[position.column as usize] = false;
    }

    EmptyMap { row_empties, col_empties }
}

fn find_new_galaxy_positions(galaxy_positions: &Vec<Position>, empty_map: EmptyMap) -> Vec<Position> {
    let mut new_galaxy_positions = Vec::new();

    for galaxy in galaxy_positions {
        // Loop over row and column and find new spot
        let mut new_row = 0;
        let mut new_col = 0;

        for i in 0..=galaxy.row {
            new_row += if empty_map.row_empties[i as usize] { 2 } else { 1 }
        }

        for i in 0..=galaxy.column {
            new_col += if empty_map.col_empties[i as usize] { 2 } else { 1 }
        }

        new_galaxy_positions.push(Position { row: new_row, column: new_col })
    }

    new_galaxy_positions
}

fn get_distance(galaxy_positions: &Vec<Position>, start: usize, end: usize) -> i32 {
    let start_galaxy = &galaxy_positions[start];
    let end_galaxy = &galaxy_positions[end];

    (end_galaxy.row - start_galaxy.row).abs() + (end_galaxy.column - start_galaxy.column).abs()
}