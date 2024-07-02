use std::collections::{BinaryHeap, HashSet};

use enums::Direction;
use structs::{Coordinate, Iteration, Node};

mod enums;
mod helpers;
mod structs;
mod tests;

const START_COORD: Coordinate = Coordinate { x: 0, y: 0 };

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let mut map = parse_map(input);

    let end_coord = Coordinate {
        x: map.len() - 1,
        y: map[map.len() - 1].len() - 1,
    };

    let mut min_heat_loss: i64 = i64::MAX;

    let mut iteration_heap: BinaryHeap<Iteration> = BinaryHeap::new();
    let mut starting_visited = HashSet::new();
    starting_visited.insert(START_COORD);

    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::E,
        moves_left: 2,
        heat_loss: 0,
        visited: starting_visited.clone(),
        path_map: Vec::new(),
    });
    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::S,
        moves_left: 2,
        heat_loss: 0,
        visited: starting_visited.clone(),
        path_map: Vec::new(),
    });

    let mut max_count = iteration_heap.len();
    let mut traversal_count = 0;

    while !iteration_heap.is_empty() {
        traverse(&mut map, &mut min_heat_loss, &mut iteration_heap, end_coord);
        traversal_count += 1;
        let queue_len = iteration_heap.len();
        println!("After traversal, {queue_len} items remaining.");
        if queue_len > max_count {
            max_count = queue_len;
        }
    }

    println!("Maximum iteration count was {max_count}, and we traversed the map {traversal_count} times.");

    min_heat_loss
}

fn part_2(input: &Vec<String>) -> i64 {
    let _map = parse_map(input);

    todo!()
}

fn parse_map(input: &Vec<String>) -> Vec<Vec<Node>> {
    let mut map = Vec::new();

    for line in input {
        let mut row = Vec::new();

        for raw_int in line.chars() {
            if raw_int == '\r' {
                continue;
            }
            row.push(Node {
                value: raw_int.to_digit(10).unwrap() as u8,
                min_heat_loss: i64::MAX,
            });
        }

        map.push(row);
    }

    map
}

fn get_new_coord(
    max: Coordinate,
    old_coord: Coordinate,
    direction: Direction,
) -> Option<Coordinate> {
    match direction {
        Direction::N => {
            let new_x = old_coord.x.checked_sub(1);
            if new_x.is_some() {
                Some(Coordinate {
                    x: new_x.unwrap(),
                    y: old_coord.y,
                })
            } else {
                None
            }
        }
        Direction::S => {
            let new_x = old_coord.x.checked_add(1);
            if new_x.is_some() && new_x.unwrap() < max.x {
                Some(Coordinate {
                    x: new_x.unwrap(),
                    y: old_coord.y,
                })
            } else {
                None
            }
        }
        Direction::W => {
            let new_y = old_coord.y.checked_sub(1);
            if new_y.is_some() {
                Some(Coordinate {
                    x: old_coord.x,
                    y: new_y.unwrap(),
                })
            } else {
                None
            }
        }
        Direction::E => {
            let new_y = old_coord.y.checked_add(1);
            if new_y.is_some() && new_y.unwrap() < max.y {
                Some(Coordinate {
                    x: old_coord.x,
                    y: new_y.unwrap(),
                })
            } else {
                None
            }
        }
    }
}

fn turn(left: bool, direction: Direction) -> Direction {
    match direction {
        Direction::N => {
            if left {
                Direction::W
            } else {
                Direction::E
            }
        }
        Direction::S => {
            if left {
                Direction::E
            } else {
                Direction::W
            }
        }
        Direction::W => {
            if left {
                Direction::S
            } else {
                Direction::N
            }
        }
        Direction::E => {
            if left {
                Direction::N
            } else {
                Direction::S
            }
        }
    }
}

fn traverse(
    map: &mut Vec<Vec<Node>>,
    min_heat_loss: &mut i64,
    iteration_queue: &mut BinaryHeap<Iteration>,
    end: Coordinate,
) {
    if iteration_queue.is_empty() {
        panic!("Iteration queue must not be empty!")
    }

    let mut starting_iter = iteration_queue.pop().unwrap();

    // Move based on indicated direction on iteration
    let new_coord = get_new_coord(
        get_max_coordinates(map),
        starting_iter.coordinate,
        starting_iter.direction,
    )
    .unwrap();
    let new_coord_node = &mut map[new_coord.x][new_coord.y];
    starting_iter.visited.insert(new_coord);
    starting_iter.path_map.push(starting_iter.direction);
    let new_heat_loss = starting_iter.heat_loss + (new_coord_node.value as i64);
    if new_heat_loss >= *min_heat_loss {
        return;
    }

    // Reached end coordinate
    if new_coord == end {
        if new_heat_loss < *min_heat_loss {
            println!("Found path with heat loss {new_heat_loss}, the map is the following:");
            print_path_map_overlay(map, &starting_iter.path_map);
            *min_heat_loss = new_heat_loss;
        }
        return;
    }

    // Queue up new iterations
    // Case: keep going
    let straight_moves_left = starting_iter.moves_left;
    if straight_moves_left > 0 {
        let straight_dir = starting_iter.direction;

        let straight_coord = get_new_coord(get_max_coordinates(map), new_coord, straight_dir);

        if let Some(straight_coord) = straight_coord {
            if !starting_iter.visited.contains(&straight_coord) {
                iteration_queue.push(Iteration {
                    coordinate: new_coord,
                    direction: straight_dir,
                    moves_left: straight_moves_left - 1,
                    heat_loss: new_heat_loss,
                    visited: starting_iter.visited.clone(),
                    path_map: starting_iter.path_map.clone(),
                });
            }
        }
    }

    // Case: Turn left or right
    for turn_left in [true, false] {
        let turn_dir = turn(turn_left, starting_iter.direction);

        let turn_coord = get_new_coord(get_max_coordinates(map), starting_iter.coordinate, turn_dir);

        if let Some(turn_coord) = turn_coord {
            if !starting_iter.visited.contains(&turn_coord) {
                iteration_queue.push(Iteration {
                    coordinate: new_coord,
                    direction: turn_dir,
                    moves_left: 2,
                    heat_loss: new_heat_loss,
                    visited: starting_iter.visited.clone(),
                    path_map: starting_iter.path_map.clone(),
                });
            }
        }
    }
}

fn get_max_coordinates<T>(map: &Vec<Vec<T>>) -> Coordinate {
    return Coordinate {
        x: map.len(),
        y: map[0].len(),
    };
}

fn print_path_map_overlay(map: &Vec<Vec<Node>>, path_map: Vec<Direction>) {
    // Initialize board
    let mut print_board: Vec<Vec<char>> = Vec::new();
    let mut curr_line: Vec<char> = Vec::new();

    // Draw border top
    for _ in 0..map[0].len() + 2 {
        curr_line.push('-');
    }
    print_board.push(curr_line);
    curr_line = Vec::new();

    // Draw actual map
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if column == 0 {
                curr_line.push('|');
            }
            curr_line.push(char::from_digit(map[row][column].value as u32, 10).unwrap());
            if column == map[row].len() - 1 {
                curr_line.push('|');
            }
        }

        print_board.push(curr_line);
        curr_line = Vec::new();
    }

    // Draw border bottom
    for _ in 0..map[0].len() + 2 {
        curr_line.push('-');
    }
    print_board.push(curr_line);

    // Start overlaying path map
    // Start at (1, 1) to avoid border
    let mut curr_pos = Coordinate { x: 1, y: 1 };
    for step in path_map {
        let dir_char: char;
        match step {
            Direction::N => {
                curr_pos.x -= 1;
                dir_char = '^';
            }
            Direction::S => {
                curr_pos.x += 1;
                dir_char = 'v';
            }
            Direction::W => {
                curr_pos.y -= 1;
                dir_char = '<';
            }
            Direction::E => {
                curr_pos.y += 1;
                dir_char = '>';
            }
        }
        print_board[curr_pos.x][curr_pos.y] = dir_char;
    }

    for row in print_board {
        for column in row {
            print!("{}", column);
        }
        println!();
    }
}
