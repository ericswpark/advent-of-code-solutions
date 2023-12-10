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

fn part_2(input: &Vec<String>) -> i32 {
    let map: Vec<Vec<char>> = get_map(input);
    let (distance_map, _) = calculate_distance_map(&map);

    let mut inside_count = 0;

    for x in 0..distance_map.len() {
        for y in 0..distance_map[x].len() {
            // Check if the position is within the loop or not, and increment in the inside count accordingly
            let pos = distance_map[x][y];

            // If the position is part of the loop, then it is obviously not the inside of the loop
            if pos.is_some() { continue }

            // Check top pipe count
            {
                let mut cur_x = x;
                let mut top_pipe_encountered = 0;
                let mut top_pipe_started = false;
                while cur_x > 0 {
                    cur_x -= 1;
                    let cur_char = map[cur_x][y];
                    if distance_map[cur_x][y].is_some() {
                        if top_pipe_started {
                            if cur_char == '|' { continue } else if ['F', '7'].contains(&cur_char) {
                                top_pipe_started = false;
                            }
                        } else {
                            top_pipe_encountered += 1;
                            top_pipe_started = true;
                        }
                    } else {
                        top_pipe_started = false;
                    }
                }
                if top_pipe_encountered % 2 == 0 { continue }
            }

            // Check bottom pipe count
            {
                let mut cur_x = x;
                let mut bottom_pipe_encountered = 0;
                let mut bottom_pipe_started = false;
                while cur_x + 1 < map.len() {
                    cur_x += 1;
                    let cur_char = map[cur_x][y];
                    if distance_map[cur_x][y].is_some() {
                        if bottom_pipe_started {
                            if cur_char == '|' { continue }
                            else if ['L', 'J'].contains(&cur_char) {
                                bottom_pipe_started = false;
                            }
                        } else {
                            bottom_pipe_encountered += 1;
                            bottom_pipe_started = true;
                        }
                    }
                    else {
                        bottom_pipe_started = false;
                    }
                }
                if bottom_pipe_encountered % 2 == 0 { continue }
            }

            // Check left pipe count
            {
                let mut cur_y = y;
                let mut left_pipe_encountered = 0;
                let mut left_pipe_started = false;
                while cur_y > 0 {
                    cur_y -= 1;
                    let cur_char = map[x][cur_y];
                    if distance_map[x][cur_y].is_some() {
                        if left_pipe_started {
                            if cur_char == '-' { continue }
                            else if ['F', 'L'].contains(&cur_char) {
                                left_pipe_started = false;
                            }
                        } else {
                            left_pipe_encountered += 1;
                            left_pipe_started = true;
                        }
                    }
                    else {
                        left_pipe_started = false;
                    }
                }
                if left_pipe_encountered % 2 == 0 { continue }
            }

            // Check right pipe count
            {
                let mut cur_y = y;
                let mut right_pipe_encountered = 0;
                let mut right_pipe_started = false;
                while cur_y + 1 < map[x].len() {
                    cur_y += 1;
                    let cur_char = map[x][cur_y];
                    if distance_map[x][cur_y].is_some() {
                        if right_pipe_started {
                            if cur_char == '-' { continue }
                            else if ['7', 'J'].contains(&cur_char) {
                                right_pipe_started = false;
                            }
                        } else {
                            right_pipe_encountered += 1;
                            right_pipe_started = true;
                        }
                    }
                    else {
                        right_pipe_started = false;
                    }
                }
                if right_pipe_encountered % 2 == 0 { continue }
            }

            // At this point we can conclude we are within the loop
            inside_count += 1
        }
    }

    inside_count
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

    (distance_map, max_walk)
}