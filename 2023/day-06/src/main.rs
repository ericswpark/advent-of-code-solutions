mod tests;

use helpers::*;
use std::time::Instant;

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
    let times = get_parsed_number_array(&input[0][5..]);
    let distances = get_parsed_number_array(&input[1][9..]);

    let mut ways_to_win_multiple = 1;

    for (time_index, time) in times.iter().enumerate() {
        let ways = get_winning_ways_for_time(time, distances[time_index]);

        ways_to_win_multiple *= ways;
    }

    ways_to_win_multiple
}

fn part_2(input: &Vec<String>) -> i64 {
    let bad_kerning_time = get_parsed_number(&input[0][5..]);
    let bad_kerning_distance = get_parsed_number(&input[1][9..]);

    get_winning_ways_for_time(&bad_kerning_time, bad_kerning_distance)
}

fn get_parsed_number(s: &str) -> i64 {
    s.split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn get_winning_ways_for_time(time: &i64, target_distance: i64) -> i64 {
    let mut ways = 0;
    for i in 0..=*time {
        let distance = calculate_distance(*time, i);
        if distance > target_distance {
            ways += 1
        }
    }
    ways
}

fn get_parsed_number_array(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect()
}

fn calculate_distance(total: i64, charge_up: i64) -> i64 {
    charge_up * (total - charge_up)
}
