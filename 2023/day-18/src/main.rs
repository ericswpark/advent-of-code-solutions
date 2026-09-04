mod tests;

use std::{collections::VecDeque, str::FromStr};

use helpers::*;

use Direction::*;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let digsteps = get_digsteps(input).expect("Failed to parse digsteps");
    
    get_inner_count(&map.terrain) as i64
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!();
}

fn get_digsteps(input: &Vec<String>) -> Result<Vec<DigStep>, String> {
    input.iter().map(|s| s.parse()).collect()
}

struct Map {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    terrain: VecDeque<VecDeque<bool>>,
}

impl Map {
    fn new() -> Self {
        Self {
            width: 1,
            height: 1,
            x: 0,
            y: 0,
            terrain: VecDeque::from([VecDeque::from([true])]),
        }
    }

    fn from_digsteps(digsteps: &[DigStep]) -> Map {
        let mut map = Self::new();
        for step in digsteps {
            for _ in 0..step.length {
                map.move_to(step.direction);
            }
        }
        map
    }

    fn move_to(&mut self, direction: Direction) {
        match direction {
            Left => {
                if self.x == 0 {
                    for row in self.terrain.iter_mut() {
                        row.push_front(false);
                    }
                    self.x += 1;
                    self.width += 1;
                }
                self.x -= 1;
            }
            Right => {
                if self.x + 1 >= self.width {
                    for row in self.terrain.iter_mut() {
                        row.push_back(false);
                    }
                    self.width += 1;
                }
                self.x += 1;
            }
            Up => {
                if self.y == 0 {
                    self.terrain.push_front(VecDeque::from(
                        std::iter::repeat(false)
                            .take(self.width)
                            .collect::<VecDeque<bool>>(),
                    ));
                    self.y += 1;
                    self.height += 1;
                }
                self.y -= 1;
            }
            Down => {
                if self.y + 1 >= self.height {
                    self.terrain.push_back(VecDeque::from(
                        std::iter::repeat(false)
                            .take(self.width)
                            .collect::<VecDeque<bool>>(),
                    ));
                    self.height += 1;
                }
                self.y += 1;
            }
        }

        self.terrain[self.y][self.x] = true;
    }
}

type Terrain = VecDeque<VecDeque<bool>>;

fn print_terrain_debug(terrain: &Terrain) {
    println!("terrain current state:");
    for row in terrain.iter() {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}

fn get_inner_count(terrain: &VecDeque<VecDeque<bool>>) -> usize {
    terrain.iter().map(|row| get_row_inside_counts(&row)).sum()
}

fn get_row_inside_counts(row: &VecDeque<bool>) -> usize {
    if row.len() == 0 {
        return 0;
    }

    let mut count = 0;
    let mut inside = false;
    let mut prev_cell: Option<bool> = None;
    let mut is_in_wall = false;

    for (x, cell) in row.iter().enumerate() {
        if *cell {
            if prev_cell.is_none() || !prev_cell.unwrap() {
                inside = !inside;
            }

            // Check for wall
            if x + 1 < row.len() && row[x + 1] {
                is_in_wall = true;
            }
        } else {
            if is_in_wall {
                inside = false;
                is_in_wall = false;
            }
        }
        if inside || *cell {
            count += 1;
        }

        prev_cell = Some(*cell);
        print!("{}", if *cell { '#' } else { '.' });
    }
    println!(" {}", count);

    count
}

#[derive(Debug, Clone)]
struct DigStep {
    direction: Direction,
    length: usize,
    color: String,
}

impl FromStr for DigStep {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err("wrong number of parts in dig step".to_string());
        }

        let direction: Direction = parts[0].parse()?;
        let length = parts[1]
            .parse::<usize>()
            .map_err(|_| "length is not usize".to_string())?;
        let color: String = parts[2].chars().skip(2).take(parts[2].len() - 3).collect();
        if color.len() != 6 {
            return Err("incomplete hex code for color in dig step".to_string());
        }

        Ok(DigStep {
            direction,
            length,
            color,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err("direction must be one char".to_string());
        }
        if let Some(c) = s.chars().next() {
            match c {
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                'U' => Ok(Self::Up),
                'D' => Ok(Self::Down),
                _ => Err("unknown direction".to_string()),
            }
        } else {
            Err("not enough characters".to_string())
        }
    }
}
