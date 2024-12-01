use std::time::Instant;

mod helpers;
mod tests;


fn main() {
    let start_time = Instant::now();
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &Vec<String>) -> i64 {
    let (mut left_list, mut right_list) = get_lists(&input);

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
    todo!()
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