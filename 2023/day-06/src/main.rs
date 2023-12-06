mod tests;

use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};



fn main() {
    let input = get_input();

    let times = get_parsed_number_array(&input[0][5..]);
    let distances = get_parsed_number_array(&input[1][9..]);

    let mut multiple_of_number_of_ways_to_win = 1;

    for (time_index, time) in times.iter().enumerate() {
        let ways = get_winning_ways_for_time(time, distances[time_index]);

        multiple_of_number_of_ways_to_win *= ways;
    }

    println!("The number of ways to win multiplied for each round is {multiple_of_number_of_ways_to_win}.");


}

fn get_winning_ways_for_time(time: &i32, target_distance: i32) -> i32 {
    let mut ways = 0;
    for i in 0..=*time {
        let distance = calculate_distance(*time, i);
        if distance > target_distance { ways += 1 }
    }
    ways
}

fn get_parsed_number_array(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|s: &str| s.parse::<i32>().unwrap()).collect()
}

fn calculate_distance(total: i32, charge_up: i32) -> i32 {
    charge_up * (total - charge_up)
}


fn get_input() -> Vec<String> {
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

    fs::read_to_string(path)
        .expect("Couldn't read input file").split("\n").map(|s| s.to_string()).collect()
}
