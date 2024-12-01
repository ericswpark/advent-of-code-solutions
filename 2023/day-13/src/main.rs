use std::cmp;

mod custom_helper;
mod tests;

use custom_helper::get_input;
use helpers::get_path_from_arg;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let patterns = parse_patterns(input);

    let mut summary = 0;

    for pattern in patterns {
        let vertical = get_vertical_reflection(&pattern, None);
        let horizontal = get_horizontal_reflection(&pattern, None);

        if vertical != -1 {
            summary += vertical;
        } else if horizontal != -1 {
            summary += 100 * horizontal;
        }
    }

    summary
}

fn part_2(input: &Vec<String>) -> i64 {
    let patterns = parse_patterns(input);

    let mut summary = 0;

    for pattern in patterns {
        // Get the original values to compare against
        let original_vertical = get_vertical_reflection(&pattern, None);
        let original_horizontal = get_horizontal_reflection(&pattern, None);

        // Test different smudges
        'outer_loop: for row in 0..pattern.len() {
            for column in 0..pattern[row].len() {
                let mut new_pattern = pattern.clone();
                new_pattern[row][column].flip();

                let new_vertical = get_vertical_reflection(&new_pattern, Some(original_vertical));
                let new_horizontal =
                    get_horizontal_reflection(&new_pattern, Some(original_horizontal));

                if new_vertical != -1 {
                    summary += new_vertical;
                    break 'outer_loop;
                } else if new_horizontal != -1 {
                    summary += 100 * new_horizontal;
                    break 'outer_loop;
                }
            }
        }
    }

    summary
}

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
enum Item {
    Rock,
    Ash,
}

impl Item {
    fn mapping(c: char) -> Self {
        match c {
            '.' => Item::Ash,
            '#' => Item::Rock,
            _ => panic!("Bad condition character!"),
        }
    }

    fn flip(&mut self) {
        if *self == Item::Ash {
            *self = Item::Rock;
        } else {
            *self = Item::Ash;
        }
    }
}

fn parse_patterns(input: &Vec<String>) -> Vec<Vec<Vec<Item>>> {
    let mut patterns = Vec::new();

    for line in input {
        let mut pattern = Vec::new();

        let rows: Vec<String> = line.split('\n').map(|s| s.to_string()).collect();

        for raw_row in rows {
            let row = parse_row(&raw_row);
            pattern.push(row);
        }

        patterns.push(pattern)
    }

    patterns
}

fn parse_row(input: &str) -> Vec<Item> {
    let mut row = Vec::new();
    for c in input.chars() {
        row.push(Item::mapping(c));
    }

    row
}

fn get_vertical_reflection(pattern: &[Vec<Item>], ignore: Option<i64>) -> i64 {
    for column in 0..pattern[0].len() - 1 {
        // Start comparing
        // Get number of columns we need to compare
        let max_compare_column_length = cmp::min(column + 1, pattern[0].len() - (column + 1));

        let mut matches = true;
        for columns_away in 0..max_compare_column_length {
            // Compare against columns across mirror
            for row in pattern {
                if row[column - columns_away] != row[column + 1 + columns_away] {
                    matches = false
                }
            }
        }

        if matches && (ignore.is_none() || ignore.unwrap() != (column + 1) as i64) {
            return (column + 1) as i64;
        }
    }

    -1
}

fn get_horizontal_reflection(pattern: &[Vec<Item>], ignore: Option<i64>) -> i64 {
    for row in 0..pattern.len() - 1 {
        // Start comparing
        // Get number of rows we need to compare
        let max_compare_row_length = cmp::min(row + 1, pattern.len() - (row + 1));

        let mut matches = true;
        for rows_away in 0..max_compare_row_length {
            // Compare against columns across mirror
            for column in 0..pattern[row].len() {
                if pattern[row - rows_away][column] != pattern[row + 1 + rows_away][column] {
                    matches = false
                }
            }
        }

        if matches && (ignore.is_none() || ignore.unwrap() != (row + 1) as i64) {
            return (row + 1) as i64;
        }
    }

    -1
}
