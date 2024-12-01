mod custom_helper;
mod tests;

use rayon::prelude::*;
use std::time::Instant;

use custom_helper::get_input;
use helpers::get_path_from_arg;

struct RangeItem {
    dest_start: i64,
    source_start: i64,
    range: i64,
}

impl RangeItem {
    fn is_in_range(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_start + self.range
    }

    fn get_mapping(&self, source: i64) -> Option<i64> {
        if !self.is_in_range(source) {
            return None;
        }

        Some(self.dest_start - self.source_start + source)
    }
}

struct RangeVec {
    ranges: Vec<RangeItem>,
}

impl RangeVec {
    fn new() -> Self {
        RangeVec { ranges: Vec::new() }
    }

    fn insert(&mut self, item: RangeItem) {
        self.ranges.push(item);
    }

    fn get(&self, source: i64) -> Option<i64> {
        for range in &self.ranges {
            if range.is_in_range(source) {
                return range.get_mapping(source);
            }
        }
        Some(source)
    }
}

struct RangeSeed {
    start: i64,
    range: i64,
}

struct Mapping {
    soil: RangeVec,
    fertilizer: RangeVec,
    water: RangeVec,
    light: RangeVec,
    temperature: RangeVec,
    humidity: RangeVec,
    location: RangeVec,
}

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
    let seeds = get_seeds(&input[0]);
    let mapping = Mapping {
        soil: get_mapping(&input[1]),
        fertilizer: get_mapping(&input[2]),
        water: get_mapping(&input[3]),
        light: get_mapping(&input[4]),
        temperature: get_mapping(&input[5]),
        humidity: get_mapping(&input[6]),
        location: get_mapping(&input[7]),
    };

    let mut lowest = i64::MAX;

    for seed in seeds {
        let location = get_location_of_seed(&mapping, seed);
        if location < lowest {
            lowest = location
        }
    }

    lowest
}

fn part_2(input: &[String]) -> i64 {
    let seeds = get_range_seeds(&get_seeds(&input[0]));
    let mapping = Mapping {
        soil: get_mapping(&input[1]),
        fertilizer: get_mapping(&input[2]),
        water: get_mapping(&input[3]),
        light: get_mapping(&input[4]),
        temperature: get_mapping(&input[5]),
        humidity: get_mapping(&input[6]),
        location: get_mapping(&input[7]),
    };

    let lowest = seeds
        .par_iter()
        .map(|seed_range| {
            let lowest: i64 = (0..seed_range.range)
                .into_par_iter()
                .map(|i| {
                    let seed = seed_range.start + i;

                    get_location_of_seed(&mapping, seed)
                })
                .min()
                .unwrap();

            let range = seed_range.range;
            println!("The seed range {range} has the lowest value {lowest}");
            lowest
        })
        .min()
        .unwrap();

    lowest
}

fn get_location_of_seed(mapping: &Mapping, seed: i64) -> i64 {
    let soil = mapping.soil.get(seed).unwrap();
    let fertilizer = mapping.fertilizer.get(soil).unwrap();
    let water = mapping.water.get(fertilizer).unwrap();
    let light = mapping.light.get(water).unwrap();
    let temperature = mapping.temperature.get(light).unwrap();
    let humidity = mapping.humidity.get(temperature).unwrap();
    mapping.location.get(humidity).unwrap()
}

fn get_range_seeds(input: &[i64]) -> Vec<RangeSeed> {
    let mut seeds = Vec::new();

    for i in 0..input.len() {
        if i % 2 == 0 {
            seeds.push(RangeSeed {
                start: input[i],
                range: input[i + 1],
            })
        }
    }

    seeds
}

fn get_seeds(input: &str) -> Vec<i64> {
    input[7..]
        .split(' ')
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect()
}

fn get_mapping(input: &str) -> RangeVec {
    let mut map: RangeVec = RangeVec::new();

    for (index, line) in input.split('\n').enumerate() {
        if index == 0 {
            continue;
        } // Skip header
        let mapping: Vec<i64> = line
            .split(' ')
            .map(|s: &str| s.parse::<i64>().unwrap())
            .collect();

        let dest_range_start = mapping[0];
        let source_range_start = mapping[1];
        let range_length = mapping[2];

        map.insert(RangeItem {
            dest_start: dest_range_start,
            source_start: source_range_start,
            range: range_length,
        });
    }

    map
}
