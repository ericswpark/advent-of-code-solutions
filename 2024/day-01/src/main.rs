use std::collections::HashMap;
use std::time::Instant;
use crate::helpers::*;

mod helpers;
mod tests;

fn main() {
    let start_time = Instant::now();
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &Vec<String>) -> i64 {
    let (mut left_list, mut right_list) = get_lists(input);

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;

    for index in 0..left_list.len() {
        let distance = (left_list[index] - right_list[index]).abs();
        total_distance += distance;
    }

    total_distance
}

fn part_2(input: &Vec<String>) -> i64 {
    let (left_list, right_list) = get_lists(input);

    let mut total_similarity = 0;

    let mut lookup = HashMap::new();

    for left_num in left_list {
        lookup.entry(left_num).or_insert_with(|| {
            let sim_count = right_list.iter().filter(|&n| *n == left_num).count();
            sim_count as i64 * left_num
        });

        total_similarity += lookup.get(&left_num).unwrap();
    }

    total_similarity
}

fn get_lists(input: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input {
        let parts = line.split("   ").collect::<Vec<&str>>();
        let left_num = parts[0].parse::<i64>().unwrap();
        let right_num = parts[1].parse::<i64>().unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    (left, right)
}
