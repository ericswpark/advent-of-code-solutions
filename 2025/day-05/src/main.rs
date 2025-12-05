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

fn part_1(input: &[String]) -> i64 {
    let data = parse_input(input, true);

    // Merge ranges
    let merged_ranges = merge_ranges(&data.ranges);

    let mut count = 0;

    for ingredient in data.available.unwrap() {
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

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            start: other.start.min(self.start),
            end: other.end.max(self.end),
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    ranges: Vec<Range>,
    available: Option<Vec<i64>>,
}

fn parse_input(input: &[String], include_available: bool) -> Data {
    let mut ranges = Vec::new();
    let mut iterator = input.iter();

    for line in iterator.by_ref().take_while(|line| !line.is_empty()) {
        let (start, end) = line.split_once('-').unwrap();
        let range = Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        };
        ranges.push(range);
    }

    if !include_available {
        return Data {
            ranges,
            available: None,
        };
    }

    let mut available = Vec::new();

    for line in iterator {
        let num = line.parse().unwrap();
        available.push(num);
    }

    Data {
        ranges,
        available: Some(available),
    }
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = Vec::new();
    ranges
        .iter()
        .for_each(|range| insert_into_merged_ranges(&mut merged_ranges, range));
    merged_ranges
}

fn insert_into_merged_ranges(merged_ranges: &mut Vec<Range>, range: &Range) {
    // Check if the new range overlaps any existing merged range
    match merged_ranges
        .iter()
        .enumerate()
        .find(|(_, merged_range)| merged_range.overlaps(range))
    {
        Some((index, _)) => {
            // Merge, remove from array
            let new_merged_range = merged_ranges[index].merge(range);
            merged_ranges.swap_remove(index);

            // Add new merged range recursively (to check for other overlaps)
            insert_into_merged_ranges(merged_ranges, &new_merged_range);
        }
        None => {
            merged_ranges.push(*range);
        }
    }
}
