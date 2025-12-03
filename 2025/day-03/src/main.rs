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

fn get_max_joltage(bank: &str) -> i64 {
    let mut max_joltage = 0;
    for first_digit_index in 0..bank.len() - 1 {
        for second_digit_index in first_digit_index + 1..bank.len() {
            let joltage = format!(
                "{}{}",
                bank.chars().nth(first_digit_index).unwrap(),
                bank.chars().nth(second_digit_index).unwrap()
            )
            .parse::<i64>()
            .unwrap();
            if joltage > max_joltage {
                max_joltage = joltage;
            }
        }
    }
    max_joltage
}

fn part_1(input: &[String]) -> i64 {
    let mut answer = 0;
    for line in input {
        let max_joltage = get_max_joltage(line);
        answer += max_joltage;
    }
    answer
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}
