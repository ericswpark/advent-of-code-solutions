use std::collections::HashMap;

mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}


fn part_1(input: &Vec<String>) -> i64 {
    let map = parse_map(input);

    let traversed_map = traverse_map(&map);

    let mut energized_sum = 0;

    // for row in &traversed_map {
    //     for col in row {
    //         if *col {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    for tile in traversed_map.iter().flatten() {
        if *tile { energized_sum += 1; }
    }

    energized_sum
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(PartialEq)]
enum Item {
    Empty,
    ForwardMirror,
    BackMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Item {
    fn mapping(c: char) -> Self {
        match c {
            '.' => Item::Empty,
            '/' => Item::ForwardMirror,
            '\\' => Item::BackMirror,
            '|' => Item::VerticalSplitter,
            '-' => Item::HorizontalSplitter,
            _ => panic!("Bad item mapping!"),
        }
    }
}

fn parse_map(input: &Vec<String>) -> Vec<Vec<Item>>{
    let mut map = Vec::new();

    for line in input {
        let mut row = Vec::new();

        for raw_item in line.chars() {
            row.push(Item::mapping(raw_item));
        }

        map.push(row);
    }

    map
}

fn traverse_map(map: &Vec<Vec<Item>>) -> Vec<Vec<bool>> {
    let mut traverse_map = vec![vec![false; map[0].len()]; map.len()];

    traverse_map_next(map, &mut traverse_map, 0, 0, Direction::E, Vec::new());

    traverse_map
}

fn get_mirrored_direction(direction: Direction, mirror: &Item) -> Direction {
    if *mirror == Item::ForwardMirror {
        return match direction {
            Direction::E => Direction::N,
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::W => Direction::S,
        }
    } else if *mirror == Item::BackMirror {
        return match direction {
            Direction::E => Direction::S,
            Direction::N => Direction::W,
            Direction::S => Direction::E,
            Direction::W => Direction::N,
        }
    } else { panic!("Not a mirror!") }
}

#[derive(PartialEq, Clone)]
struct Iteration {
    x: usize,
    y: usize,
    direction: Direction,
}

fn traverse_map_next(map: &Vec<Vec<Item>>, traverse_map: &mut Vec<Vec<bool>>, x: i64, y: i64, direction: Direction, mut loop_detect: Vec<Iteration>) {
    let mut x = x;
    let mut y = y;

    // Check if we're stuck in a loop and exit early if so
    let current_coordinate = Iteration { x: x as usize, y: y as usize, direction };
    if loop_detect.contains(&current_coordinate) { return }
    else {
        loop_detect.push(current_coordinate);
    }

    // Set current position to energized
    traverse_map[x as usize][y as usize] = true;

    match direction {
        Direction::N => { x -= 1; }
        Direction::S => { x += 1; }
        Direction::W => { y -= 1; }
        Direction::E => { y += 1; }
    }

    // Check bounds
    if x >= 0 && x < map.len() as i64 && y >= 0 && y < map[x as usize].len() as i64 {
        // Check what we're currently on and continue on to next iteration
        match map[x as usize][y as usize] {
            Item::Empty => {
                traverse_map_next(map, traverse_map, x, y, direction, loop_detect.clone());
            }
            Item::ForwardMirror | Item::BackMirror => {
                traverse_map_next(map, traverse_map, x, y, get_mirrored_direction(direction, &map[x as usize][y as usize]), loop_detect.clone());
            }
            Item::VerticalSplitter => {
                if direction == Direction::N || direction == Direction::S {
                    traverse_map_next(map, traverse_map, x, y, direction, loop_detect.clone());
                } else {
                    traverse_map_next(map, traverse_map, x, y, Direction::N, loop_detect.clone());
                    traverse_map_next(map, traverse_map, x, y, Direction::S, loop_detect.clone());
                }
            }
            Item::HorizontalSplitter => {
                if direction == Direction::E || direction == Direction::W {
                    traverse_map_next(map, traverse_map, x, y, direction, loop_detect.clone());
                } else {
                    traverse_map_next(map, traverse_map, x, y, Direction::W, loop_detect.clone());
                    traverse_map_next(map, traverse_map, x, y, Direction::E, loop_detect.clone());
                }
            }
        }
    }
}