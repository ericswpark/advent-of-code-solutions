use std::cmp::PartialEq;
use std::time::Instant;

use crate::Direction::*;
use helpers::*;

mod tests;

fn main() {
    let input = get_input(&get_path_from_arg());

    let start_time = Instant::now();
    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &[String]) -> i64 {
    let (map, guard_row, guard_col) = parse_map(input);
    let walk_map: Vec<Vec<bool>> = get_walk_map(&map, guard_row, guard_col);

    // Count all trues in walk_map
    let mut count = 0;
    for row in walk_map {
        for x in row {
            if x {
                count += 1;
            }
        }
    }

    count
}

fn part_2(input: &[String]) -> i64 {
    let (map, guard_row, guard_col) = parse_map(input);
    let walk_map: Vec<Vec<bool>> = get_walk_map(&map, guard_row, guard_col);

    let mut obstruction_count = 0;

    for (row_idx, row) in walk_map.iter().enumerate() {
        for (col_idx, spot) in row.iter().enumerate() {
            // Skip start as we don't want to get caught
            if row_idx == guard_row && col_idx == guard_col {
                continue;
            }

            // Skip spots that the guard doesn't pass to speed up search
            if !spot {
                continue;
            }

            let mut new_map = map.clone();
            new_map[row_idx][col_idx] = true;
            if will_loop_in_map(&new_map, guard_row, guard_col) {
                obstruction_count += 1;
            }
        }
    }

    obstruction_count
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Left => Up,
            Right => Down,
            Up => Right,
            Down => Left,
        }
    }
}

/// Returns map where true means the spot has an obstacle, false otherwise
/// Also returns guard position
/// Starting direction is assumed to be north (heading up)
fn parse_map(input: &[String]) -> (Vec<Vec<bool>>, usize, usize) {
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;

    for (x, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (y, ch) in line.chars().enumerate() {
            match ch {
                '.' => row.push(false),
                '#' => row.push(true),
                '^' => {
                    row.push(false);
                    guard_x = x;
                    guard_y = y;
                }
                _ => {
                    panic!("{} is not possible, bad input?", ch)
                }
            }
        }
        map.push(row);
    }

    (map, guard_x, guard_y)
}

/// Get walk map based on map and guard starting position
fn get_walk_map(map: &[Vec<bool>], start_row: usize, start_col: usize) -> Vec<Vec<bool>> {
    let mut walk_map: Vec<Vec<bool>> = vec![vec![false; map.len()]; map[0].len()];

    let mut guard_direction = Up;
    let mut guard_row = start_row;
    let mut guard_col = start_col;

    // Mark starting position on walk_map
    walk_map[guard_row][guard_col] = true;

    loop {
        let check_next = get_next_pos(map, guard_direction, guard_row, guard_col);
        if check_next.is_none() {
            break;
        }

        let (next_row, next_col) = check_next.unwrap();

        // Check if next position is an obstruction and rotate otherwise
        if map[next_row][next_col] {
            guard_direction = guard_direction.rotate_right();
        } else {
            guard_row = next_row;
            guard_col = next_col;
            walk_map[guard_row][guard_col] = true;
        }
    }

    walk_map
}

/// Get walk map based on map and guard starting position
fn will_loop_in_map(map: &[Vec<bool>], start_row: usize, start_col: usize) -> bool {
    let mut walk_map: Vec<Vec<Option<Direction>>> = vec![vec![None; map.len()]; map[0].len()];

    let mut guard_row = start_row;
    let mut guard_col = start_col;

    // Mark starting position on walk_map
    let mut cur_direction = Up;
    walk_map[guard_row][guard_col] = Some(cur_direction);

    loop {
        let check_next = get_next_pos(map, cur_direction, guard_row, guard_col);
        if check_next.is_none() {
            return false;
        }

        let (next_row, next_col) = check_next.unwrap();

        // Check if next position is an obstruction and rotate otherwise
        if map[next_row][next_col] {
            cur_direction = cur_direction.rotate_right();
        } else {
            // Check if we've been here using the same direction before
            if let Some(direction) = walk_map[next_row][next_col] {
                if direction == cur_direction {
                    return true;
                }
            }
            walk_map[next_row][next_col] = Some(cur_direction);
            guard_row = next_row;
            guard_col = next_col;
        }
    }
}

fn get_next_pos(
    map: &[Vec<bool>],
    direction: Direction,
    row: usize,
    col: usize,
) -> Option<(usize, usize)> {
    match direction {
        Left => Some((row, col.checked_sub(1)?)),
        Right => {
            if col + 1 >= map[row].len() {
                None
            } else {
                Some((row, col + 1))
            }
        }
        Up => Some((row.checked_sub(1)?, col)),
        Down => {
            if row + 1 >= map.len() {
                None
            } else {
                Some((row + 1, col))
            }
        }
    }
}
