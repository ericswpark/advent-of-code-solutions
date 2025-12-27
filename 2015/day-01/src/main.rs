mod tests;

use helpers::*;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let input = &input[0];

    get_level(input)
}

fn part_2(input: &[String]) -> i64 {
    let input = &input[0];

    get_basement_position(input) + 1
}

fn get_level(input: &str) -> i64 {
    let mut level = 0;
    for c in input.chars() {
        if c == '(' {
            level += 1
        } else if c == ')' {
            level -= 1
        }
    }
    level
}

fn get_basement_position(input: &str) -> i64 {
    let mut level = 0;
    for (index, c) in input.chars().enumerate() {
        if c == '(' {
            level += 1
        } else if c == ')' {
            level -= 1
        }

        if level < 0 {
            return index as i64;
        }
    }
    panic!("No basement level")
}
