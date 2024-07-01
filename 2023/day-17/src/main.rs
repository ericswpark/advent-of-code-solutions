use std::collections::VecDeque;

use enums::Direction;
use structs::{Coordinate, Iteration, Node};

mod helpers;
mod tests;
mod structs;
mod enums;

const START_COORD: Coordinate = Coordinate {
    x: 0,
    y: 0,
};

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

    let mut heat_losses: Vec<i64> = Vec::new();

    let mut iteration_queue: VecDeque<Iteration> = VecDeque::new();

    iteration_queue.push_back(Iteration { coordinate: START_COORD, direction: Direction::E, moves_left: 3, heat_loss: 0 });
    iteration_queue.push_back(Iteration { coordinate: START_COORD, direction: Direction::S, moves_left: 3, heat_loss: 0 });

    let mut max_count = iteration_queue.len();
    let mut traversal_count = 0;

    while !iteration_queue.is_empty() {
        traverse(&mut map, &mut heat_losses, &mut iteration_queue, end_coord);
        traversal_count += 1;
        let queue_len = iteration_queue.len();
        println!("After traversal, {queue_len} items remaining.");
        if queue_len > max_count {
            max_count = queue_len;
        }
    }

    println!("Maximum iteration count was {max_count}, and we traversed the map {traversal_count} times.");

    *heat_losses.iter().min().unwrap()
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
            if raw_int == '\r' { continue; }
            row.push(Node{ value: raw_int.to_digit(10).unwrap() as u8, min_heat_loss: i64::MAX});
        }

        map.push(row);
    }

    map
}


fn get_new_coord(max: Coordinate, old_coord: Coordinate, direction: Direction) -> Option<Coordinate> {
    match direction {
        Direction::N => {
            let new_x = old_coord.x.checked_sub(1);
            if new_x.is_some() { Some(Coordinate { x: new_x.unwrap(), y: old_coord.y }) } else { None }
        }
        Direction::S => {
            let new_x = old_coord.x.checked_add(1);
            if new_x.is_some() && new_x.unwrap() < max.x { Some(Coordinate { x: new_x.unwrap(), y: old_coord.y }) } else { None }
        }
        Direction::W => {
            let new_y = old_coord.y.checked_sub(1);
            if new_y.is_some() { Some(Coordinate { x: old_coord.x, y: new_y.unwrap() }) } else { None }
        }
        Direction::E => {
            let new_y = old_coord.y.checked_add(1);
            if new_y.is_some() && new_y.unwrap() < max.y { Some(Coordinate { x: old_coord.x, y: new_y.unwrap() }) } else { None }
        }
    }
}

fn turn(left: bool, direction: Direction) -> Direction {
    match direction {
        Direction::N => { if left { Direction::W } else { Direction::E } }
        Direction::S => { if left { Direction::E } else { Direction::W } }
        Direction::W => { if left { Direction::S } else { Direction::N } }
        Direction::E => { if left { Direction::N } else { Direction::S } }
    }
}


fn traverse(
    map: &mut Vec<Vec<Node>>,
    heat_losses: &mut Vec<i64>,
    iteration_queue: &mut VecDeque<Iteration>,
    end: Coordinate,
) {
    if iteration_queue.is_empty() {
        panic!("Iteration queue must not be empty!")
    }

    let starting_iter = iteration_queue.pop_front().unwrap();

    // Move based on indicated direction on iteration
    let new_coord = get_new_coord(get_max_coordinates(map), starting_iter.coordinate, starting_iter.direction).unwrap();
    let new_coord_node = &mut map[new_coord.x][new_coord.y];
    let new_heat_loss = starting_iter.heat_loss + (new_coord_node.value as i64);
    if new_heat_loss < new_coord_node.min_heat_loss {
        new_coord_node.min_heat_loss = new_heat_loss;
    } else {
        return;
    }

    // Reached end coordinate
    if new_coord == end {
        heat_losses.push(new_heat_loss);
        return;
    }

    // Queue up new iterations
    // Case: keep going
    let straight_moves_left = starting_iter.moves_left;
    if straight_moves_left > 0 {
        let straight_dir = starting_iter.direction;

        if get_new_coord(get_max_coordinates(map), new_coord, straight_dir).is_some() {
            iteration_queue.push_back(Iteration { coordinate: new_coord, direction: straight_dir, moves_left: straight_moves_left - 1, heat_loss: new_heat_loss });
        }
    }

    // Case: Turn left or right
    for turn_left in [true, false] {
        let turn_dir = turn(turn_left, starting_iter.direction);

        if get_new_coord(get_max_coordinates(map), starting_iter.coordinate, turn_dir).is_some() {
            iteration_queue.push_back(Iteration { coordinate: new_coord, direction: turn_dir, moves_left: 3, heat_loss: new_heat_loss });
        }
    }
}

fn get_max_coordinates<T>(map: &Vec<Vec<T>>) -> Coordinate {
    return Coordinate { x: map.len(), y: map[0].len() };
}