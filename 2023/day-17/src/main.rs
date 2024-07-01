use std::collections::{HashSet, VecDeque};

mod helpers;
mod tests;
mod structs;
mod enums;

use structs::Coordinate;
use enums::Direction;
use crate::structs::Iteration;


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
    let map = parse_map(input);

    let end_coord = Coordinate {
        x: map.len() - 1,
        y: map[map.len() - 1].len() - 1,
    };

    let mut heat_losses: Vec<i64> = Vec::new();

    let mut iteration_queue: VecDeque<Iteration> = VecDeque::new();

    iteration_queue.push_back(Iteration {coordinate: START_COORD, direction: Direction::E, moves_left: 3, heat_loss: 0, visited: HashSet::new()});
    iteration_queue.push_back(Iteration {coordinate: START_COORD, direction: Direction::S, moves_left: 3, heat_loss: 0, visited: HashSet::new()});

    let mut max_count = iteration_queue.len();
    let mut traversal_count = 0;

    while !iteration_queue.is_empty() {
        traverse(&map, &mut heat_losses, &mut iteration_queue, end_coord);
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

fn parse_map(input: &Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    for line in input {
        let mut row = Vec::new();

        for raw_int in line.chars() {
            if raw_int == '\r' { continue }
            row.push(raw_int.to_digit(10).unwrap() as u8);
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
    map: &Vec<Vec<u8>>,
    heat_losses: &mut Vec<i64>,
    iteration_queue: &mut VecDeque<Iteration>,
    end: Coordinate,
) {
    if iteration_queue.is_empty() {
        panic!("Iteration queue must not be empty!")
    }

    let mut current = iteration_queue.pop_front().unwrap();

    // If we're on the end coordinate, add current heat loss to Vec and quit
    if current.coordinate == end {
        heat_losses.push(current.heat_loss);
        return;
    }

    // If our current coordinate is in the set of past coordinates, we've looped and should return early
    if current.visited.contains(&current.coordinate) {
        let visited_count = current.visited.len();
        println!("Detected loop after {visited_count} moves.");
        return;
    } else {
        current.visited.insert(current.coordinate);
    }

    // Case: keep going
    if current.moves_left > 0 {
        let new_coord = get_new_coord(get_max_coordinates(map), current.coordinate, current.direction);

        if let Some(new_coord) = new_coord {
            iteration_queue.push_back(Iteration {coordinate: new_coord, direction: current.direction, moves_left: current.moves_left - 1, heat_loss: current.heat_loss + map[new_coord.x][new_coord.y] as i64, visited: current.visited.clone()});
        }
    }

    // Case: Turn left or right
    for turn_left in [true, false] {
        let new_dir = turn(turn_left, current.direction);
        let new_coord = get_new_coord(get_max_coordinates(map), current.coordinate, new_dir);

        if let Some(new_coord) = new_coord {
            iteration_queue.push_back(Iteration {coordinate: new_coord, direction: current.direction, moves_left: 3, heat_loss: current.heat_loss + map[new_coord.x][new_coord.y] as i64, visited: current.visited.clone()});
        }
    }
}

fn get_max_coordinates(map: &Vec<Vec<u8>>) -> Coordinate {
    return Coordinate {x: map.len(), y: map[0].len() }
}