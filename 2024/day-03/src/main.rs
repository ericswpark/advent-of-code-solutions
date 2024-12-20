use regex::Regex;
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
    let mut total = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for line in input {
        for (_, [lhs, rhs]) in re.captures_iter(line).map(|c| c.extract()) {
            total += lhs.parse::<i64>().unwrap() * rhs.parse::<i64>().unwrap();
        }
    }
    total
}

fn part_2(input: &Vec<String>) -> i64 {
    let mut total = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don?'?t?\(\)").unwrap();

    let mut enabled = true;
    for line in input {
        for item in re.captures_iter(line) {
            match item[0].chars().next().unwrap() {
                'm' => {
                    if enabled {
                        let lhs = item[1].parse::<i64>().unwrap();
                        let rhs = item[2].parse::<i64>().unwrap();
                        total += lhs * rhs;
                    }
                }
                'd' => {
                    if item[0] == *"do()" {
                        enabled = true;
                    } else if item[0] == *"don't()" {
                        enabled = false;
                    }
                }
                _ => {
                    panic!("Not possible")
                }
            }
        }
    }
    total
}
