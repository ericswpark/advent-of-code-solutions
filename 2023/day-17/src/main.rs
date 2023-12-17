#![recursion_limit="100000000"]

use std::collections::HashSet;

mod helpers;
mod tests;

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

    // We call twice because there are two starting directions
    recursive_traverse(&map, &mut heat_losses, START_COORD, end_coord, Direction::E, 3, 0, HashSet::new());
    recursive_traverse(&map, &mut heat_losses, START_COORD, end_coord, Direction::S, 3, 0, HashSet::new());

    *heat_losses.iter().min().unwrap()
}

fn part_2(input: &Vec<String>) -> i64 {
    let map = parse_map(input);

    todo!()
}

fn parse_map(input: &Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    for line in input {
        let mut row = Vec::new();

        for raw_int in line.chars() {
            row.push(raw_int.to_digit(10).unwrap() as u8);
        }

        map.push(row);
    }

    map
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

fn get_new_coord(old_coord: Coordinate, direction: Direction) -> Option<Coordinate> {
    match direction {
        Direction::N => {
            let new_x = old_coord.x.checked_sub(1);
            if new_x.is_some() { Some(Coordinate { x: new_x.unwrap(), y: old_coord.y }) } else { None }
        }
        Direction::S => {
            let new_x = old_coord.x.checked_add(1);
            if new_x.is_some() { Some(Coordinate { x: new_x.unwrap(), y: old_coord.y }) } else { None }
        }
        Direction::W => {
            let new_y = old_coord.y.checked_sub(1);
            if new_y.is_some() { Some(Coordinate { x: old_coord.x, y: new_y.unwrap() }) } else { None }
        }
        Direction::E => {
            let new_y = old_coord.y.checked_add(1);
            if new_y.is_some() { Some(Coordinate { x: old_coord.x, y: new_y.unwrap() }) } else { None }
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

#[derive(Clone, Hash, PartialEq, Eq)]
struct Iteration {
    coordinate: Coordinate,
    direction: Direction,
}

fn recursive_traverse(
    map: &Vec<Vec<u8>>,
    heat_losses: &mut Vec<i64>,
    current: Coordinate,
    end: Coordinate,
    previous_direction: Direction,
    moves_left_in_direction: u8,
    current_heat_loss: i64,
    mut loop_detect: HashSet<Iteration>,
) {
    // If we're on the end coordinate, add current heat loss to Vec and quit
    if current == end {
        heat_losses.push(current_heat_loss);
        return;
    }

    // Check if we are in a loop
    let current_loop_iteration = Iteration { coordinate: current, direction: previous_direction };
    if loop_detect.contains(&current_loop_iteration) {
        return;
    } else {
        loop_detect.insert(current_loop_iteration);
    }

    // Case: keep going
    if moves_left_in_direction > 0 {
        let new_coord = get_new_coord(current, previous_direction);

        if new_coord.is_some() {
            recursive_traverse(map,
                               heat_losses,
                               new_coord.unwrap(),
                               end,
                               previous_direction,
                               moves_left_in_direction - 1,
                               current_heat_loss,
                               loop_detect.clone(),
            )
        }
    }

    // Case: Turn left or right
    for turn_dir in [true, false] {
        let new_dir = turn(turn_dir, previous_direction);
        let new_coord = get_new_coord(current, new_dir);

        if new_coord.is_some() {
            recursive_traverse(map,
                               heat_losses,
                               new_coord.unwrap(),
                               end,
                               new_dir,
                               3,
                               current_heat_loss,
                                loop_detect.clone()
            )
        }
    }
}
