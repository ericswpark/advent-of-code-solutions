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

fn part_1(input: &[String]) -> i64 {
    let arrays: Vec<Vec<String>> = parse_to_arrays(input);

    let number_arrays = &arrays[0..arrays.len() - 1];
    let operations = arrays.last().unwrap();

    let mut total = 0;

    for (index, operation) in operations.iter().enumerate() {
        match operation.chars().next().unwrap() {
            '+' => {
                let result: i64 = number_arrays
                    .iter()
                    .map(|numbers| numbers[index].parse::<i64>().unwrap())
                    .sum();
                total += result;
            }
            '*' => {
                let result: i64 = number_arrays
                    .iter()
                    .map(|numbers| numbers[index].parse::<i64>().unwrap())
                    .product();
                total += result;
            }
            _ => unreachable!("Unsupported operation"),
        }
    }

    total
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

fn parse_to_arrays(input: &[String]) -> Vec<Vec<String>> {
    input.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}
