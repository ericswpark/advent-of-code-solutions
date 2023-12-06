mod tests;

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};


fn main() {
    let input = get_input(&*get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("The number of ways to win multiplied for each round is {part_1_answer}.");

    let part_2_answer = part_2(&input);
    println!("The number of ways to win (with bad kerning) is {part_2_answer}.");
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
    s.split_whitespace().collect::<String>().parse::<i64>().unwrap()
}

fn get_winning_ways_for_time(time: &i64, target_distance: i64) -> i64 {
    let mut ways = 0;
    for i in 0..=*time {
        let distance = calculate_distance(*time, i);
        if distance > target_distance { ways += 1 }
    }
    ways
}

fn get_parsed_number_array(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|s: &str| s.parse::<i64>().unwrap()).collect()
}

fn calculate_distance(total: i64, charge_up: i64) -> i64 {
    charge_up * (total - charge_up)
}

fn get_path_from_arg() -> String {
    let mut args: Vec<String> = env::args().collect();

    let mut path: &mut String = &mut String::new();

    if args.len() < 2 {
        print!("Enter path to file: ");
        stdout().flush().expect("Cannot flush buffer");

        stdin().read_line(path).expect("Cannot process input");
        if let Some('\n')=path.chars().next_back() {
            path.pop();
        }
        if let Some('\r')=path.chars().next_back() {
            path.pop();
        }
    } else {
        path = &mut args[1];
    }

    path.to_owned()
}


fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Couldn't read input file").split("\n").map(|s| s.to_string()).collect()
}
