use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

use enums::Direction;
use structs::{Coordinate, Iteration, Node};

mod enums;
mod structs;
mod tests;

use helpers::*;

const START_COORD: Coordinate = Coordinate { x: 0, y: 0 };

fn main() {
    let start_time = Instant::now();
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &Vec<String>) -> i64 {
    let mut map = parse_map(input);

    let end_coord = Coordinate {
        x: map.len() - 1,
        y: map[map.len() - 1].len() - 1,
    };

    let mut iteration_heap: BinaryHeap<Iteration> = BinaryHeap::new();

    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::E,
        straight_moves: 1,
        heat_loss: 0,
        path_map: Vec::new(),
    });
    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::S,
        straight_moves: 1,
        heat_loss: 0,
        path_map: Vec::new(),
    });

    traverse(&mut map, &mut iteration_heap, end_coord)
}

fn part_2(input: &Vec<String>) -> i64 {
    let mut map = parse_map(input);

    let end_coord = Coordinate {
        x: map.len() - 1,
        y: map[map.len() - 1].len() - 1,
    };

    let mut iteration_heap: BinaryHeap<Iteration> = BinaryHeap::new();

    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::E,
        straight_moves: 1,
        heat_loss: 0,
        path_map: Vec::new(),
    });
    iteration_heap.push(Iteration {
        coordinate: START_COORD,
        direction: Direction::S,
        straight_moves: 1,
        heat_loss: 0,
        path_map: Vec::new(),
    });

    traverse_ultra(&mut map, &mut iteration_heap, end_coord)
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
    move_by: Option<usize>,
) -> Option<Coordinate> {
    let move_by = move_by.unwrap_or(1);

    match direction {
        Direction::N => {
            let new_x = old_coord.x.checked_sub(move_by);
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
            let new_x = old_coord.x.checked_add(move_by);
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
            let new_y = old_coord.y.checked_sub(move_by);
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
            let new_y = old_coord.y.checked_add(move_by);
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

fn traverse(
    map: &mut Vec<Vec<Node>>,
    iteration_queue: &mut BinaryHeap<Iteration>,
    end: Coordinate,
) -> i64 {
    let mut visited = HashSet::new();

    loop {
        let mut starting_iter = iteration_queue.pop().unwrap();

        // Move based on indicated direction on iteration
        let new_coord = get_new_coord(
            get_max_coordinates(map),
            starting_iter.coordinate,
            starting_iter.direction,
            None,
        )
        .unwrap();

        let visited_item = (
            new_coord,
            starting_iter.direction,
            starting_iter.straight_moves,
        );

        if visited.contains(&visited_item) {
            continue;
        }
        visited.insert(visited_item);
        starting_iter.path_map.push(starting_iter.direction);
        let new_coord_node = &mut map[new_coord.x][new_coord.y];
        let new_heat_loss = starting_iter.heat_loss + (new_coord_node.value as i64);

        // Reached end coordinate
        if new_coord == end {
            println!("Found path with heat loss {new_heat_loss}, the map is the following:");
            print_path_map_overlay(map, &starting_iter.path_map);
            return new_heat_loss;
        }

        // Queue up new iterations
        // Case: keep going
        if starting_iter.straight_moves < 3 {
            let straight_dir = starting_iter.direction;

            if get_new_coord(get_max_coordinates(map), new_coord, straight_dir, None).is_some() {
                iteration_queue.push(Iteration {
                    coordinate: new_coord,
                    direction: straight_dir,
                    straight_moves: starting_iter.straight_moves + 1,
                    heat_loss: new_heat_loss,
                    path_map: starting_iter.path_map.clone(),
                });
            }
        }

        // Case: Turn left or right
        let starting_iter_direction = starting_iter.direction;
        for turn_dir in [
            starting_iter_direction.left(),
            starting_iter_direction.right(),
        ] {
            if get_new_coord(
                get_max_coordinates(map),
                starting_iter.coordinate,
                turn_dir,
                None,
            )
            .is_some()
            {
                iteration_queue.push(Iteration {
                    coordinate: new_coord,
                    direction: turn_dir,
                    straight_moves: 1,
                    heat_loss: new_heat_loss,
                    path_map: starting_iter.path_map.clone(),
                });
            }
        }
    }
}

fn traverse_ultra(
    map: &mut Vec<Vec<Node>>,
    iteration_queue: &mut BinaryHeap<Iteration>,
    end: Coordinate,
) -> i64 {
    let mut visited = HashSet::new();

    loop {
        let mut starting_iter = iteration_queue.pop().unwrap();

        // Move based on indicated direction on iteration
        let new_coord = get_new_coord(
            get_max_coordinates(map),
            starting_iter.coordinate,
            starting_iter.direction,
            None,
        )
        .unwrap();

        let visited_item = (
            new_coord,
            starting_iter.direction,
            starting_iter.straight_moves,
        );

        if visited.contains(&visited_item) {
            continue;
        }
        visited.insert(visited_item);
        starting_iter.path_map.push(starting_iter.direction);
        let new_coord_node = &mut map[new_coord.x][new_coord.y];
        let new_heat_loss = starting_iter.heat_loss + (new_coord_node.value as i64);

        // Reached end coordinate
        if new_coord == end && starting_iter.straight_moves >= 4 {
            println!("Found path with heat loss {new_heat_loss}, the map is the following:");
            print_path_map_overlay(map, &starting_iter.path_map);
            return new_heat_loss;
        }

        // Queue up new iterations
        // Case: keep going
        if starting_iter.straight_moves < 10 {
            let straight_dir = starting_iter.direction;

            if get_new_coord(get_max_coordinates(map), new_coord, straight_dir, None).is_some() {
                iteration_queue.push(Iteration {
                    coordinate: new_coord,
                    direction: straight_dir,
                    straight_moves: starting_iter.straight_moves + 1,
                    heat_loss: new_heat_loss,
                    path_map: starting_iter.path_map.clone(),
                });
            }
        }

        // Case: Turn left or right
        if starting_iter.straight_moves >= 4 {
            let starting_iter_direction = starting_iter.direction;
            for turn_dir in [
                starting_iter_direction.left(),
                starting_iter_direction.right(),
            ] {
                if get_new_coord(
                    get_max_coordinates(map),
                    starting_iter.coordinate,
                    turn_dir,
                    None,
                )
                .is_some()
                {
                    iteration_queue.push(Iteration {
                        coordinate: new_coord,
                        direction: turn_dir,
                        straight_moves: 1,
                        heat_loss: new_heat_loss,
                        path_map: starting_iter.path_map.clone(),
                    });
                }
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

fn print_path_map_overlay(map: &Vec<Vec<Node>>, path_map: &Vec<Direction>) {
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
