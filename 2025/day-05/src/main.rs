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
    let data = parse_input(input, true);

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
    let data = parse_input(input, false);
    let merged_ranges = merge_ranges(&data.ranges);

    merged_ranges.iter().map(|range| range.count()).sum()
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

    fn count(&self) -> i64 {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone)]
struct Data {
    ranges: Vec<Range>,
    available: Vec<i64>,
}

fn parse_input(input: &[String], include_available: bool) -> Data {
    let mut ranges = Vec::new();
    let mut available = Vec::new();
    let mut is_on_ranges = true;

    for line in input {
        if line == "" {
            is_on_ranges = false;

            if !include_available {
                break;
            }

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
    ranges
        .iter()
        .for_each(|range| insert_into_merged_ranges(&mut merged_ranges, range));
    merged_ranges
}

fn insert_into_merged_ranges(merged_ranges: &mut Vec<Range>, range: &Range) {
    let mut existing_range: Option<usize> = None;

    for (i, merged_range) in merged_ranges.iter().enumerate() {
        if merged_range.includes(range.start) || merged_range.includes(range.end) {
            existing_range = Some(i);
            break;
        }
    }

    match existing_range {
        Some(index) => {
            // Remove from array
            let merged_range = &mut merged_ranges[index];
            let new_merged_range = Range {
                start: merged_range.start.min(range.start),
                end: merged_range.end.max(range.end),
            };

            merged_ranges.remove(index);

            // Add back again
            insert_into_merged_ranges(merged_ranges, &new_merged_range);
        }
        None => {
            merged_ranges.push(*range);
        }
    }
}
