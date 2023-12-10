mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

struct Position {
    x: usize,
    y: usize,
}

fn part_1(input: &Vec<String>) -> usize {
    let map: Vec<Vec<char>> = get_map(input);

    // Find starting position
    let start = get_starting_position(&map);

    // Initialize distance map
    let mut distance_map: Vec<Vec<Option<usize>>> = vec![vec![None; map[0].len()]; map.len()];
    distance_map[start.x][start.y] = Some(0);

    // Start walking from the start position
    let mut max_walk = 0;
    loop {
        let mut has_walked_next = false;

        // Find all points with the current walk
        let current_positions = get_all_positions(&distance_map, max_walk);

        for position in current_positions {
            // Search in a circle for None values and see if they connect
            // Left
            if position.y > 0 &&
                distance_map[position.x][position.y - 1] == None &&
                ['-', 'L', 'F'].contains(&map[position.x][position.y - 1])  {
                has_walked_next = true;
                distance_map[position.x][position.y - 1] = Some(max_walk + 1);
            }
            // Top
            if position.x > 0 &&
                distance_map[position.x - 1][position.y] == None &&
                ['|', '7', 'F'].contains(&map[position.x - 1][position.y])  {
                has_walked_next = true;
                distance_map[position.x - 1][position.y] = Some(max_walk + 1);
            }
            // Right
            if position.y + 1 < map[position.x].len() &&
                distance_map[position.x][position.y + 1] == None &&
                ['-', '7', 'J'].contains(&map[position.x][position.y + 1])  {
                has_walked_next = true;
                distance_map[position.x][position.y + 1] = Some(max_walk + 1);
            }
            // Bottom
            if position.x + 1 < map.len() &&
                distance_map[position.x + 1][position.y] == None &&
                ['|', 'L', 'J'].contains(&map[position.x + 1][position.y])  {
                has_walked_next = true;
                distance_map[position.x + 1][position.y] = Some(max_walk + 1);
            }
        }

        if has_walked_next {
            max_walk += 1
        } else { break }
    }

    max_walk
}

fn part_2(input: &Vec<String>) -> i32 {
    // TODO: implement
    0
}

fn get_map(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input {
        map.push(line.chars().collect());
    }
    map
}

fn get_starting_position(map: &Vec<Vec<char>>) -> Position {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == 'S' { return Position { x, y } }
        }
    }
    panic!("Bad map!")
}

fn get_all_positions(distance_map: &Vec<Vec<Option<usize>>>, distance: usize) -> Vec<Position> {
    let mut positions = Vec::new();
    for x in 0..distance_map.len() {
        for y in 0..distance_map[x].len() {
            if distance_map[x][y] == Some(distance) { positions.push(Position {x, y}) }
        }
    }

    positions
}