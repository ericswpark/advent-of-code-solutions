use std::{
    collections::{HashMap, HashSet},
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
    let map = construct_map(input);
    let antenna_positions = get_antenna_positions(&map);
    let mut antinode_coordinates: HashSet<Coordinate> = HashSet::new();

    for letter in antenna_positions.keys() {
        let antennas = antenna_positions.get(letter).unwrap();

        for antenna in antennas {
            for other_antenna in antennas {
                if antenna == other_antenna {
                    continue;
                }

                let antinode = get_antinode(*antenna, *other_antenna);

                if let Some(antinode) = antinode
                    && antinode.x < map[0].len()
                    && antinode.y < map.len()
                {
                    antinode_coordinates.insert(antinode);
                }
            }
        }
    }

    antinode_coordinates.len() as i64
}

fn part_2(input: &[String]) -> i64 {
    let map = construct_map(input);
    let antenna_positions = get_antenna_positions(&map);
    let mut antinode_coordinates: HashSet<Coordinate> = HashSet::new();

    for letter in antenna_positions.keys() {
        let antennas = antenna_positions.get(letter).unwrap();

        for antenna in antennas {
            for other_antenna in antennas {
                if antenna == other_antenna {
                    continue;
                }

                let new_antinodes = get_extended_antinodes(
                    *antenna,
                    *other_antenna,
                    Coordinate {
                        x: map[0].len(),
                        y: map.len(),
                    },
                );
                antinode_coordinates.extend(new_antinodes);

                // Insert ourselves as well
                antinode_coordinates.insert(*antenna);
            }
        }
    }

    antinode_coordinates.len() as i64
}

fn construct_map(input: &[String]) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input {
        map.push(line.trim().chars().collect());
    }
    map
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

fn get_antenna_positions(map: &[Vec<char>]) -> HashMap<char, HashSet<Coordinate>> {
    let mut position_mapping = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                position_mapping
                    .entry(cell)
                    .or_insert(HashSet::new())
                    .insert(Coordinate { x, y });
            }
        }
    }

    position_mapping
}

/// Returns antinode coordinate produced by two antennas.
///
/// Note that we only get the antinode coordinate if you start from the antenna
/// and jump over the other one to get to the antinode, so to get all antinodes
/// from two antennas you need to call this function again with the antennas
/// swapped.
///
/// The antinode is guaranteed to have correct `usize` values but may be out of
/// bounds in terms of the map. If the `usize` constraint is not met then this
/// function returns `None`.
fn get_antinode(antenna: Coordinate, other_antenna: Coordinate) -> Option<Coordinate> {
    let (x1, y1) = (antenna.x as isize, antenna.y as isize);
    let (x2, y2) = (other_antenna.x as isize, other_antenna.y as isize);

    let new_x = x2 - (x1 - x2);
    let new_y = y2 - (y1 - y2);

    if new_x >= 0 && new_y >= 0 {
        return Some(Coordinate::new(new_x as usize, new_y as usize));
    }

    None
}

/// Returns extended antinode coordinates produced by two antennas.
///
/// Note that we only get the antinode coordinate if you start from the antenna
/// and jump over the other one to get to the antinode, so to get all antinodes
/// from two antennas you need to call this function again with the antennas
/// swapped.
///
/// Antinodes are guaranteed to be in bounds of the map dimensions given.
fn get_extended_antinodes(
    antenna: Coordinate,
    other_antenna: Coordinate,
    map_bounds: Coordinate,
) -> HashSet<Coordinate> {
    let mut antinodes = HashSet::new();

    let (x1, y1) = (antenna.x as isize, antenna.y as isize);
    let (x2, y2) = (other_antenna.x as isize, other_antenna.y as isize);

    let dx = x1 - x2;
    let dy = y1 - y2;

    let mut new_x = x2 - dx;
    let mut new_y = y2 - dy;

    while new_x >= 0 && new_x < map_bounds.x as isize && new_y >= 0 && new_y < map_bounds.y as isize
    {
        antinodes.insert(Coordinate::new(new_x as usize, new_y as usize));
        new_x -= dx;
        new_y -= dy;
    }

    antinodes
}
