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
    let (_, max_walk) = calculate_distance_map(&map);

    max_walk
}

trait BoolExt {
    fn flip(&mut self);
}

impl BoolExt for bool {
    fn flip(&mut self) {
        *self = !*self;
    }
}

fn part_2(input: &Vec<String>) -> i32 {
    let map: Vec<Vec<char>> = get_map(input);
    let (distance_map, _) = calculate_distance_map(&map);
    let mut map = get_solved_map(&map, &distance_map);

    let mut inside_loop = false;
    let mut inside_count = 0;
    let mut prev_char = '-';

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if distance_map[x][y].is_some() {
                let pipe = map[x][y];

                match pipe {
                    '|' => { inside_loop.flip() }
                    'F' | 'L' => {
                        prev_char = pipe;
                    }
                    'J' => {
                        if prev_char == 'F' { inside_loop.flip() }
                    }
                    '7' => {
                        if prev_char == 'L' { inside_loop.flip() }
                    }
                    _ => {}
                }
            } else {
                if inside_loop {
                    inside_count += 1;
                    map[x][y] = '*';
                } else {
                    map[x][y] = ' ';
                }
            }

        }
    }

    inside_count
}

fn get_solved_map(map: &Vec<Vec<char>>, distance_map: &Vec<Vec<Option<usize>>>) -> Vec<Vec<char>> {
    let mut solved_map = map.clone();

    // Find position of starting block
    let start_pos = get_starting_position(&map);

    // Left-up configuration
    if start_pos.x > 0 && start_pos.y > 0 &&
        distance_map[start_pos.x - 1][start_pos.y].is_some() &&
        distance_map[start_pos.x][start_pos.y - 1].is_some() &&
        ['|', 'F', '7'].contains(&map[start_pos.x - 1][start_pos.y]) &&
        ['-', 'F', 'L'].contains(&map[start_pos.x][start_pos.y - 1]) {
        solved_map[start_pos.x][start_pos.y] = 'J';
    }
    // Right-up configuration
    else if start_pos.x > 0 && start_pos.y + 1 < map[start_pos.x].len() &&
        distance_map[start_pos.x - 1][start_pos.y].is_some() &&
        distance_map[start_pos.x][start_pos.y + 1].is_some() &&
        ['|', 'F', '7'].contains(&map[start_pos.x - 1][start_pos.y]) &&
        ['-', 'J', '7'].contains(&map[start_pos.x][start_pos.y - 1]) {
        solved_map[start_pos.x][start_pos.y] = 'L';
    }
    // Right-down configuration
    else if start_pos.x + 1 < map.len() && start_pos.y + 1 < map[start_pos.x].len() &&
        distance_map[start_pos.x + 1][start_pos.y].is_some() &&
        distance_map[start_pos.x][start_pos.y + 1].is_some() &&
        ['|', 'J', 'L'].contains(&map[start_pos.x + 1][start_pos.y]) &&
        ['-', 'J', '7'].contains(&map[start_pos.x][start_pos.y + 1]) {
        solved_map[start_pos.x][start_pos.y] = 'F';
    }
    // Left-down configuration
    else if start_pos.x + 1 < map.len() && start_pos.y > 0 &&
        distance_map[start_pos.x + 1][start_pos.y].is_some() &&
        distance_map[start_pos.x][start_pos.y - 1].is_some() &&
        ['|', 'J', 'L'].contains(&map[start_pos.x + 1][start_pos.y]) &&
        ['-', 'L', 'F'].contains(&map[start_pos.x][start_pos.y - 1]) {
        solved_map[start_pos.x][start_pos.y] = '7';
    }
    // Left-right configuration
    else if start_pos.y > 0 && start_pos.y + 1 < map[start_pos.x].len() &&
        distance_map[start_pos.x][start_pos.y - 1].is_some() &&
        distance_map[start_pos.x][start_pos.y + 1].is_some() &&
        ['-', 'L', 'F'].contains(&map[start_pos.x][start_pos.y - 1]) &&
        ['-', '7', 'J'].contains(&map[start_pos.x][start_pos.y + 1]) {
        solved_map[start_pos.x][start_pos.y] = '-';
    }
    // Up-down configuration
    else if start_pos.x > 0 && start_pos.x + 1 < map.len() &&
        distance_map[start_pos.x - 1][start_pos.y].is_some() &&
        distance_map[start_pos.x + 1][start_pos.y].is_some() &&
        ['|', '7', 'F'].contains(&map[start_pos.x - 1][start_pos.y]) &&
        ['-', 'L', 'J'].contains(&map[start_pos.x + 1][start_pos.y]) {
        solved_map[start_pos.x][start_pos.y] = '|';
    }

    solved_map
}

fn _print_distance_map(input: &Vec<Vec<Option<usize>>>) {
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y].is_some() {
                print!("{:0>2} ", input[x][y].unwrap());
            } else {
                print!("__ ");
            }

        }
        println!()
    }
}

fn _print_map(input: &Vec<Vec<char>>) {
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            print!("{}", input[x][y]);
        }
        println!();
    }
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

fn calculate_distance_map(map: &Vec<Vec<char>>) -> (Vec<Vec<Option<usize>>>, usize) {
    let start = get_starting_position(map);

    let mut distance_map: Vec<Vec<Option<usize>>> = vec![vec![None; map[0].len()]; map.len()];
    distance_map[start.x][start.y] = Some(0);

    // Start walking from the start position
    let mut max_walk = 0;
    loop {
        let mut has_walked_next = false;

        // Find all points with the current walk
        let current_positions = get_all_positions(&distance_map, max_walk);

        for position in current_positions {
            let mut search_left = false;
            let mut search_right = false;
            let mut search_up = false;
            let mut search_down = false;
            match map[position.x][position.y] {
                '-' => {
                    search_left = true;
                    search_right = true;
                },
                '|' => {
                    search_up = true;
                    search_down = true;
                },
                'F' => {
                    search_right = true;
                    search_down = true;
                },
                'L' => {
                    search_up = true;
                    search_right = true;
                },
                '7' => {
                    search_left = true;
                    search_down = true;
                },
                'J' => {
                    search_up = true;
                    search_left = true;
                },
                'S' => {
                    // Starting point, search everywhere
                    search_left = true;
                    search_up = true;
                    search_down = true;
                    search_right = true;
                }
                _ => panic!("Bad mapping!"),
            }

            // Search in a circle for None values and see if they connect
            // Left
            if search_left && position.y > 0 &&
                distance_map[position.x][position.y - 1] == None &&
                ['-', 'L', 'F'].contains(&map[position.x][position.y - 1])  {
                has_walked_next = true;
                distance_map[position.x][position.y - 1] = Some(max_walk + 1);
            }
            // Up
            if search_up && position.x > 0 &&
                distance_map[position.x - 1][position.y] == None &&
                ['|', '7', 'F'].contains(&map[position.x - 1][position.y])  {
                has_walked_next = true;
                distance_map[position.x - 1][position.y] = Some(max_walk + 1);
            }
            // Right
            if search_right && position.y + 1 < map[position.x].len() &&
                distance_map[position.x][position.y + 1] == None &&
                ['-', '7', 'J'].contains(&map[position.x][position.y + 1])  {
                has_walked_next = true;
                distance_map[position.x][position.y + 1] = Some(max_walk + 1);
            }
            // Down
            if search_down && position.x + 1 < map.len() &&
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

    (distance_map, max_walk)
}