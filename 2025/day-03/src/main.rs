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
    let mut answer = 0;
    for line in input {
        let max_joltage = get_max_joltage(line, 2);
        answer += max_joltage;
    }
    answer
}

fn get_max_joltage(bank: &str, battery_count: usize) -> i64 {
    // Split bank into individual batteries
    let batteries: Vec<u8> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut batteries_used = 0;
    let mut from_index = 0;
    let mut joltage = 0;

    while batteries_used < battery_count {
        // Get range of batteries we can consider
        let batteries_needed_next_iter = battery_count - batteries_used - 1;
        let up_to_index = batteries.len() - 1 - batteries_needed_next_iter;

        // Find the biggest battery within the range
        let biggest_battery_index = batteries
            .iter()
            .skip(from_index)
            .take(up_to_index - from_index + 1)
            .position(|&battery| {
                battery
                    == *batteries
                        .iter()
                        .skip(from_index)
                        .take(up_to_index - from_index + 1)
                        .max()
                        .unwrap()
            })
            .unwrap()
            + from_index;

        // Append to joltage
        joltage = joltage * 10 + batteries[biggest_battery_index] as i64;

        // Update last used index and battery count
        from_index = biggest_battery_index + 1;
        batteries_used += 1;
    }
    joltage
}

fn part_2(input: &[String]) -> i64 {
    let mut answer = 0;
    for line in input {
        let max_joltage = get_max_joltage(line, 12);
        answer += max_joltage;
    }
    answer
}
