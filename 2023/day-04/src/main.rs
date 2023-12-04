use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() {
    let input = get_input();

    let points_worth = get_points_worth(&input);
    let card_instances_sum = get_card_instances_sum(&input);

    println!("The cards are worth {points_worth} points.");
    println!("There are {card_instances_sum} scratchcards.");
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
        .expect("Couldn't read input file").split('\n').map(|s| s.to_string()).collect()
}


fn get_points_worth(input: &Vec<String>) -> i32 {
    let mut points_sum: i32 = 0;
    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        // let card_id = parts[0][5..].parse::<i32>().unwrap();
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let winning_numbers: Vec<i32> = numbers[0].split(' ').filter(|&x| !x.is_empty()).map(|s: &str| s.parse::<i32>().unwrap() ).collect();
        let elf_numbers: Vec<i32> = numbers[1].split(' ').filter(|&x| !x.is_empty()).map(|s: &str| s.parse::<i32>().unwrap() ).collect();

        // Get number of matches
        let mut match_count = 0;
        for i in 0..elf_numbers.len() {
            for j in 0..winning_numbers.len() {
                if elf_numbers.get(i) == winning_numbers.get(j) {
                    match_count += 1;
                    break;
                }
            }
        }

        points_sum += if match_count >= 1 { 2_i32.checked_pow(match_count - 1).unwrap() } else { 0 };
    }
    points_sum
}

fn get_card_instances_sum(input: &Vec<String>) -> i32 {
    let mut cards_count_by_id: Vec<i32> = vec![0; get_card_count(input) as usize];

    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        let card_id = parts[0][5..].trim().parse::<i32>().unwrap();
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let winning_numbers: Vec<i32> = numbers[0].split(' ').filter(|&x| !x.is_empty()).map(|s: &str| s.parse::<i32>().unwrap() ).collect();
        let elf_numbers: Vec<i32> = numbers[1].split(' ').filter(|&x| !x.is_empty()).map(|s: &str| s.parse::<i32>().unwrap() ).collect();

        cards_count_by_id[card_id as usize - 1] += 1;

        // Get number of matches
        let mut match_count = 0;
        for i in 0..elf_numbers.len() {
            for j in 0..winning_numbers.len() {
                if elf_numbers.get(i) == winning_numbers.get(j) {
                    match_count += 1;
                    break;
                }
            }
        }

        // Increment number of matches, repeat for number of matches in current ID
        for _ in 0..cards_count_by_id[card_id as usize - 1] {
            for i in 0..match_count {
                cards_count_by_id[card_id as usize + i] += 1;
            }
        }
    }

    cards_count_by_id.iter().sum()
}

fn get_card_count(input: &Vec<String>) -> i32 {
    let mut last_card_id = 0;
    for line in input {
        let card_id = line.split(": ").collect::<Vec<_>>()[0][5..].trim().parse::<i32>().unwrap();
        last_card_id = card_id;
    }

    last_card_id
}