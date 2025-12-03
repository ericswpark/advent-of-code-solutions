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
    let mut first_digit_index = bank.len() - 2;
    let mut second_digit_index = bank.len() - 1;

    // Figure out how big the first digit can be
    for index in (0..first_digit_index - 1).rev() {
        if bank.chars().nth(index).unwrap().to_digit(10).unwrap()
            > bank
                .chars()
                .nth(first_digit_index)
                .unwrap()
                .to_digit(10)
                .unwrap()
        {
            first_digit_index = index;
        }
    }

    // Figure out how big the second digit can be
    for index in (first_digit_index + 1..second_digit_index - 1).rev() {
        if bank.chars().nth(index).unwrap().to_digit(10).unwrap()
            > bank
                .chars()
                .nth(second_digit_index)
                .unwrap()
                .to_digit(10)
                .unwrap()
        {
            second_digit_index = index;
        }
    }

    // Return parsed integer
    format!(
        "{}{}",
        bank.chars().nth(first_digit_index).unwrap(),
        bank.chars().nth(second_digit_index).unwrap()
    )
    .parse()
    .unwrap()
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
