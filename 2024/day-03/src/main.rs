use std::time::Instant;
use regex::Regex;

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
        for (_, [lhs, rhs]) in re.captures_iter(&line).map(|c| c.extract()) {
            total += lhs.parse::<i64>().unwrap() * rhs.parse::<i64>().unwrap();
        }
    }
    total
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}