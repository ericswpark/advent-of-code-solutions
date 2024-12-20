use std::time::Instant;

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

fn part_1(input: &Vec<String>) -> i64 {
    let map = get_map(input);
    let mut count = 0;

    for (row_idx, row) in map.iter().enumerate() {
        for col_idx in 0..row.len() {
            for dir in SearchDirection::VALUES {
                if search_map(&map, row_idx, col_idx, dir) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_2(input: &Vec<String>) -> i64 {
    let map = get_map(input);
    let mut count = 0;

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if search_map_x_mas(&map, row_idx, col_idx) {
                count += 1;
            }
        }
    }
    count
}

enum SearchDirection {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl SearchDirection {
    const VALUES: [Self; 8] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::LeftUp,
        Self::RightUp,
        Self::LeftDown,
        Self::RightDown,
    ];
}

fn get_map(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input {
        map.push(line.chars().collect());
    }

    map
}

const PATTERN: &str = "XMAS";
const X_MAS_PATTERN: &str = "MAS";

fn search_map(map: &[Vec<char>], row_idx: usize, col_idx: usize, dir: SearchDirection) -> bool {
    match dir {
        SearchDirection::Up => {
            if row_idx < PATTERN.len() - 1 {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx - cur_idx][col_idx] != PATTERN.as_bytes()[cur_idx] as char {
                    return false;
                }
            }
            true
        }
        SearchDirection::Down => {
            if row_idx + PATTERN.len() > map.len() {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx + cur_idx][col_idx] != PATTERN.as_bytes()[cur_idx] as char {
                    return false;
                }
            }
            true
        }
        SearchDirection::Left => {
            if col_idx < PATTERN.len() - 1 {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx][col_idx - cur_idx] != PATTERN.as_bytes()[cur_idx] as char {
                    return false;
                }
            }
            true
        }
        SearchDirection::Right => {
            if col_idx + PATTERN.len() > map[row_idx].len() {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx][col_idx + cur_idx] != PATTERN.as_bytes()[cur_idx] as char {
                    return false;
                }
            }
            true
        }
        SearchDirection::LeftUp => {
            if col_idx < PATTERN.len() - 1 || row_idx < PATTERN.len() - 1 {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx - cur_idx][col_idx - cur_idx] != PATTERN.as_bytes()[cur_idx] as char
                {
                    return false;
                }
            }
            true
        }
        SearchDirection::RightUp => {
            if col_idx + PATTERN.len() > map[row_idx].len() || row_idx < PATTERN.len() - 1 {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx - cur_idx][col_idx + cur_idx] != PATTERN.as_bytes()[cur_idx] as char
                {
                    return false;
                }
            }
            true
        }
        SearchDirection::LeftDown => {
            if col_idx < PATTERN.len() - 1 || row_idx + PATTERN.len() > map.len() {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx + cur_idx][col_idx - cur_idx] != PATTERN.as_bytes()[cur_idx] as char
                {
                    return false;
                }
            }
            true
        }
        SearchDirection::RightDown => {
            if col_idx + PATTERN.len() > map[row_idx].len() || row_idx + PATTERN.len() > map.len() {
                return false;
            }
            for cur_idx in 0..PATTERN.len() {
                if map[row_idx + cur_idx][col_idx + cur_idx] != PATTERN.as_bytes()[cur_idx] as char
                {
                    return false;
                }
            }
            true
        }
    }
}

fn search_map_x_mas(map: &[Vec<char>], row_idx: usize, col_idx: usize) -> bool {
    if row_idx == 0 || row_idx + 1 >= map.len() || col_idx == 0 || col_idx + 1 >= map[row_idx].len()
    {
        return false;
    }
    if map[row_idx][col_idx] != 'A' {
        return false;
    }

    let crossdown = format!(
        "{}{}{}",
        map[row_idx - 1][col_idx - 1],
        'A',
        map[row_idx + 1][col_idx + 1]
    );
    let crossup = format!(
        "{}{}{}",
        map[row_idx + 1][col_idx - 1],
        'A',
        map[row_idx - 1][col_idx + 1]
    );

    let mut count = 0;
    if crossdown == X_MAS_PATTERN {
        count += 1;
    }
    if crossdown.chars().rev().collect::<String>() == X_MAS_PATTERN {
        count += 1;
    }
    if crossup == X_MAS_PATTERN {
        count += 1;
    }
    if crossup.chars().rev().collect::<String>() == X_MAS_PATTERN {
        count += 1;
    }

    count == 2
}
