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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

fn part_1(input: &[String]) -> i64 {
    let mut answer = 0;
    let ranges = input[0]
        .split(',')
        .map(|range| {
            let (start, end) = range
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>();

    for range in ranges {
        for number in range.start..=range.end {
            if !check_validity(number) {
                answer += number;
            }
        }
    }

    answer
}

fn check_validity(number: i64) -> bool {
    let number_string = number.to_string();

    // If the length is not even, it cannot repeat
    if number_string.len() % 2 != 0 {
        return true;
    }

    for index in 0..(number_string.len() / 2) {
        if number_string.chars().nth(index).unwrap()
            != number_string
                .chars()
                .nth(number_string.len() / 2 + index)
                .unwrap()
        {
            return true;
        }
    }

    false
}

fn part_2(input: &[String]) -> i64 {
    todo!()
}
