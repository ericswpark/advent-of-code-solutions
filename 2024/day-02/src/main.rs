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

fn part_1(input: &Vec<String>) -> i64 {
    let mut safe_count = 0;

    for report in input {
        let levels: Vec<u32> = report.split(' ').map(|s| s.parse::<u32>().unwrap()).collect();
        if check_report_safety(&levels) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}


fn check_report_safety(levels: &[u32]) -> bool {
    let mut sorted: Vec<u32> = levels.to_vec();
    sorted.sort();
    let mut sorted_reverse = sorted.clone();
    sorted_reverse.sort_by(|a, b| b.cmp(a));

    if !(levels == sorted || levels == sorted_reverse) {
        // Not safe, not strictly increasing or decreasing
        return false;
    }

    let mut is_safe = true;

    for index in 0..levels.len() - 1 {
        let current = levels[index];
        let next = levels[index + 1];
        let diff = current.abs_diff(next);

        if diff <= 0 || diff > 3 {
            is_safe = false;
            break;
        }
    }

    is_safe
}