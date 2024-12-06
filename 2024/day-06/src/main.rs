use std::time::Instant;

use helpers::*;
use crate::Direction::*;

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
    let mut walk_map: Vec<Vec<bool>> = vec![vec![false; map.len()]; map[0].len()];

    let mut guard_row = guard_row;
    let mut guard_col = guard_col;
    let mut guard_direction = Up;

    // Mark starting position on walk_map
    walk_map[guard_row][guard_col] = true;

    loop {
        let mut left_map = false;
        let mut next_row = guard_row;
        let mut next_col = guard_col;

        match guard_direction {
            Left => {
                if next_col == 0 {
                    left_map = true;
                } else {
                    next_col -= 1;
                }
            }
            Right => {
                if next_col + 1 >= map[next_row].len() {
                    left_map = true;
                } else {
                    next_col += 1;
                }
            }
            Up => {
                if next_row == 0 {
                    left_map = true;
                } else {
                    next_row -= 1;
                }
            }
            Down => {
                if next_row + 1 >= map.len() {
                    left_map = true;
                } else {
                    next_row += 1;
                }
            }
        }

        if left_map {
            break;
        }

        // Check if next position is an obstruction and rotate otherwise
        if map[next_row][next_col] {
            guard_direction = guard_direction.rotate_right();
        } else {
            guard_row = next_row;
            guard_col = next_col;
            walk_map[guard_row][guard_col] = true;
        }
    }

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
    todo!()
}

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
fn parse_map(input: &[String]) -> (Vec<Vec<bool>>, usize, usize)  {
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
                _ => { panic!("{} is not possible, bad input?", ch) }
            }
        }
        map.push(row);
    }

    (map, guard_x, guard_y)
}
