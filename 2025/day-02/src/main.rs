use helpers::*;

mod tests;

aoc_main!();

fn get_ranges(str: &str) -> Vec<Range> {
    str.split(',')
        .map(|range| {
            let (start, end) = range
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>()
}
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

fn part_1(input: &[String]) -> i64 {
    let mut answer = 0;
    let ranges = get_ranges(&input[0]);

    for range in ranges {
        for number in range.start..=range.end {
            if !part_1_check_validity(number) {
                answer += number;
            }
        }
    }

    answer
}

fn part_1_check_validity(number: i64) -> bool {
    let number_string = number.to_string();

    // If the length is not even, it cannot repeat
    if !number_string.len().is_multiple_of(2) {
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

fn part_2_check_validity(number: i64) -> bool {
    let number_string = number.to_string();

    for check_length in (1..=(number_string.len() / 2)).rev() {
        // Get all segments to compare
        let segments = number_string
            .as_bytes()
            .chunks(check_length)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
            .collect::<Vec<&str>>();

        // Check if all segments are equal
        let first = segments[0];
        if segments.iter().all(|segment| *segment == first) {
            return false;
        }
    }

    true
}

fn part_2(input: &[String]) -> i64 {
    let mut answer = 0;
    let ranges = get_ranges(&input[0]);

    for range in ranges {
        for number in range.start..=range.end {
            if !part_2_check_validity(number) {
                answer += number;
            }
        }
    }

    answer
}
