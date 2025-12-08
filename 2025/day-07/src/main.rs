use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

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
    let map = parse_map(input);

    // Queue of beams
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    // Push back starting position
    queue.push_back(find_tachyon_position(&map));

    // Keep track of visited coordinates
    let mut visited: HashSet<Coordinate> = HashSet::new();

    // Keep track of split count
    let mut split = 0;

    // Process queue of beams
    while let Some(coord) = queue.pop_front() {
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);

        // We are a beam, check the next coordinate if we are within bounds
        if coord.row_idx + 1 < map.len() {
            let new_coord = Coordinate {
                row_idx: coord.row_idx + 1,
                col_idx: coord.col_idx,
            };
            match map[new_coord.row_idx][new_coord.col_idx] {
                Cell::Empty => {
                    // We can advance, push a new beam into the queue
                    queue.push_back(new_coord);
                }
                Cell::TachyonBeam => unreachable!("Multiple tachyon beam emitters"),
                Cell::Splitter => {
                    // We split into two beams, check bounds before pushing
                    if coord.col_idx + 1 < map[coord.row_idx + 1].len() {
                        queue.push_back(Coordinate {
                            row_idx: coord.row_idx + 1,
                            col_idx: coord.col_idx + 1,
                        });
                    }
                    if coord.col_idx > 0 {
                        queue.push_back(Coordinate {
                            row_idx: coord.row_idx + 1,
                            col_idx: coord.col_idx - 1,
                        });
                    }
                    split += 1;
                }
            }
        }
    }

    // Count number of splits
    split
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    TachyonBeam,
    Splitter,
}

impl Cell {
    fn new(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            'S' => Cell::TachyonBeam,
            '^' => Cell::Splitter,
            _ => panic!("Invalid cell character"),
        }
    }
}

/// Parses map from given input
fn parse_map(input: &[String]) -> Vec<Vec<Cell>> {
    input
        .iter()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    row_idx: usize,
    col_idx: usize,
}

fn find_tachyon_position(map: &[Vec<Cell>]) -> Coordinate {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == Cell::TachyonBeam {
                return Coordinate { row_idx, col_idx };
            }
        }
    }
    unreachable!("No tachyon beam emitter found");
}
