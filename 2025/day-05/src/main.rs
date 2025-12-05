use std::{collections::HashSet, time::Instant};

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
    let data = parse_input(input);

    // Merge ranges
    let merged_ranges = merge_ranges(&data.ranges);

    let mut count = 0;

    for ingredient in data.available {
        for range in &merged_ranges {
            if range.includes(ingredient) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn includes(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug, Clone)]
struct Data {
    ranges: Vec<Range>,
    available: Vec<i64>,
}

fn parse_input(input: &[String]) -> Data {
    let mut ranges = Vec::new();
    let mut available = Vec::new();
    let mut is_on_ranges = true;

    for line in input {
        if line == "" {
            is_on_ranges = false;
            continue;
        }

        match is_on_ranges {
            true => {
                let (start, end) = line.split_once('-').unwrap();
                let range = Range {
                    start: start.parse().unwrap(),
                    end: end.parse().unwrap(),
                };
                ranges.push(range);
            }
            false => {
                let num = line.parse().unwrap();
                available.push(num);
            }
        }
    }

    Data { ranges, available }
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        let mut merged = false;

        // Check if range start or end is in any merged range
        for merged_range in merged_ranges.iter_mut() {
            if merged_range.includes(range.start) || merged_range.includes(range.end) {
                merged_range.start = merged_range.start.min(range.start);
                merged_range.end = merged_range.end.max(range.end);
                merged = true;
                break;
            }
        }

        if !merged {
            merged_ranges.push(*range);
        }
    }

    merged_ranges
}
