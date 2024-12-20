use std::collections::HashSet;

mod tests;

use helpers::*;
use rayon::prelude::*;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let map = parse_map(input);

    let traversed_map = traverse_map(&map, 0, 0, Direction::E);

    get_energized_sum(&traversed_map)
}

fn get_energized_sum(traverse_map: &[Vec<bool>]) -> i64 {
    let mut energized_sum = 0;

    for tile in traverse_map.iter().flatten() {
        if *tile {
            energized_sum += 1;
        }
    }

    energized_sum
}
fn part_2(input: &Vec<String>) -> i64 {
    let map = parse_map(input);

    // Four sides
    let top_max = (0..map[0].len())
        .into_par_iter()
        .map(|col| get_energized_sum(&traverse_map(&map, 0, col as i64, Direction::S)))
        .max()
        .unwrap();
    let bottom_max = (0..map[map.len() - 1].len())
        .into_par_iter()
        .map(|col| {
            get_energized_sum(&traverse_map(
                &map,
                (map.len() - 1) as i64,
                col as i64,
                Direction::N,
            ))
        })
        .max()
        .unwrap();
    let left_max = (0..map.len())
        .into_par_iter()
        .map(|row| get_energized_sum(&traverse_map(&map, row as i64, 0, Direction::E)))
        .max()
        .unwrap();
    let right_max = (0..map.len())
        .into_par_iter()
        .map(|row| {
            get_energized_sum(&traverse_map(
                &map,
                row as i64,
                (map[0].len() - 1) as i64,
                Direction::W,
            ))
        })
        .max()
        .unwrap();

    // Add cases for corners
    let top_left = get_energized_sum(&traverse_map(&map, 0, 0, Direction::E));
    let top_right = get_energized_sum(&traverse_map(
        &map,
        0,
        (map[0].len() - 1) as i64,
        Direction::W,
    ));
    let bottom_left =
        get_energized_sum(&traverse_map(&map, (map.len() - 1) as i64, 0, Direction::E));
    let bottom_right = get_energized_sum(&traverse_map(
        &map,
        (map.len() - 1) as i64,
        (map[0].len() - 1) as i64,
        Direction::W,
    ));

    *[
        top_max,
        bottom_max,
        left_max,
        right_max,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    ]
    .iter()
    .max()
    .unwrap()
}

#[derive(PartialEq, Clone, Eq, Hash, Copy)]
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

fn parse_map(input: &Vec<String>) -> Vec<Vec<Item>> {
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

fn traverse_map(map: &Vec<Vec<Item>>, x: i64, y: i64, direction: Direction) -> Vec<Vec<bool>> {
    let mut traverse_map = vec![vec![false; map[0].len()]; map.len()];

    let mut loop_detect = HashSet::new();
    traverse_map_next(map, &mut traverse_map, x, y, direction, &mut loop_detect);

    traverse_map
}

fn get_mirrored_direction(direction: Direction, mirror: &Item) -> Direction {
    if *mirror == Item::ForwardMirror {
        match direction {
            Direction::E => Direction::N,
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::W => Direction::S,
        }
    } else if *mirror == Item::BackMirror {
        match direction {
            Direction::E => Direction::S,
            Direction::N => Direction::W,
            Direction::S => Direction::E,
            Direction::W => Direction::N,
        }
    } else {
        panic!("Not a mirror!")
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Iteration {
    x: usize,
    y: usize,
    direction: Direction,
}

fn in_bounds(map: &[Vec<Item>], x: i64, y: i64) -> bool {
    x >= 0 && x < map.len() as i64 && y >= 0 && y < map[x as usize].len() as i64
}

fn next_direction(x: i64, y: i64, direction: Direction) -> (i64, i64) {
    match direction {
        Direction::N => (x - 1, y),
        Direction::S => (x + 1, y),
        Direction::W => (x, y - 1),
        Direction::E => (x, y + 1),
    }
}

fn traverse_map_next(
    map: &Vec<Vec<Item>>,
    traverse_map: &mut Vec<Vec<bool>>,
    x: i64,
    y: i64,
    direction: Direction,
    loop_detect: &mut HashSet<Iteration>,
) {
    let mut x = x;
    let mut y = y;

    // Check if we're stuck in a loop and exit early if so
    let current_coordinate = Iteration {
        x: x as usize,
        y: y as usize,
        direction,
    };
    if loop_detect.contains(&current_coordinate) {
        return;
    } else {
        loop_detect.insert(current_coordinate);
    }

    // Set current position to energized
    traverse_map[x as usize][y as usize] = true;

    match map[x as usize][y as usize] {
        Item::Empty => {
            (x, y) = next_direction(x, y, direction);
            if in_bounds(map, x, y) {
                traverse_map_next(map, traverse_map, x, y, direction, loop_detect);
            }
        }
        Item::ForwardMirror | Item::BackMirror => {
            let next_mirrored_direction =
                get_mirrored_direction(direction, &map[x as usize][y as usize]);
            (x, y) = next_direction(x, y, next_mirrored_direction);
            if in_bounds(map, x, y) {
                traverse_map_next(
                    map,
                    traverse_map,
                    x,
                    y,
                    next_mirrored_direction,
                    loop_detect,
                );
            }
        }
        Item::VerticalSplitter => {
            if [Direction::N, Direction::S].contains(&direction) {
                (x, y) = next_direction(x, y, direction);
                if in_bounds(map, x, y) {
                    traverse_map_next(map, traverse_map, x, y, direction, loop_detect);
                }
            } else {
                let (north_x, north_y) = next_direction(x, y, Direction::N);
                if in_bounds(map, north_x, north_y) {
                    traverse_map_next(
                        map,
                        traverse_map,
                        north_x,
                        north_y,
                        Direction::N,
                        loop_detect,
                    );
                }
                let (south_x, south_y) = next_direction(x, y, Direction::S);
                if in_bounds(map, south_x, south_y) {
                    traverse_map_next(
                        map,
                        traverse_map,
                        south_x,
                        south_y,
                        Direction::S,
                        loop_detect,
                    );
                }
            }
        }
        Item::HorizontalSplitter => {
            if [Direction::E, Direction::W].contains(&direction) {
                (x, y) = next_direction(x, y, direction);
                if in_bounds(map, x, y) {
                    traverse_map_next(map, traverse_map, x, y, direction, loop_detect);
                }
            } else {
                let (west_x, west_y) = next_direction(x, y, Direction::W);
                if in_bounds(map, west_x, west_y) {
                    traverse_map_next(map, traverse_map, west_x, west_y, Direction::W, loop_detect);
                }
                let (east_x, east_y) = next_direction(x, y, Direction::E);
                if in_bounds(map, east_x, east_y) {
                    traverse_map_next(map, traverse_map, east_x, east_y, Direction::E, loop_detect);
                }
            }
        }
    }
}
