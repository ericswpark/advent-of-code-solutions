use rayon::prelude::*;
use std::time::Instant;

use helpers::*;

struct RangeItem {
    dest_start: i64,
    source_start: i64,
    range: i64
}

impl RangeItem {
    fn is_in_range(&self, source: i64) -> bool {
        source >= self.source_start && source <= self.source_start + self.range - 1
    }

    fn get_mapping(&self, source: i64) -> Option<i64> {
        if !self.is_in_range(source) { return None; }

        Some(self.dest_start - self.source_start + source)
    }
}

struct RangeVec {
    ranges: Vec<RangeItem>
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
                return range.get_mapping(source)
            }
        }
        Some(source)
    }
}

struct RangeSeed {
    start: i64,
    range: i64,
}

fn main() {
    let input = get_input(&get_path_from_arg());

    let start_time = Instant::now();

    let seeds_part_1 = get_seeds(&input[0]);

    let seeds_part_2 = get_range_seeds(&seeds_part_1);

    let soil_mapping = get_mapping(&input[1]);
    let fertilizer_mapping = get_mapping(&input[2]);
    let water_mapping = get_mapping(&input[3]);
    let light_mapping = get_mapping(&input[4]);
    let temperature_mapping = get_mapping(&input[5]);
    let humidity_mapping = get_mapping(&input[6]);
    let location_mapping = get_mapping(&input[7]);

    let mut lowest = i64::MAX;

    for seed in seeds_part_1 {
        let location = get_location_of_seed(&soil_mapping, &fertilizer_mapping, &water_mapping, &light_mapping, &temperature_mapping, &humidity_mapping, &location_mapping, seed);

        if location < lowest { lowest = location }
    }

    println!("Part 1: The lowest location number is {lowest}.");
    println!("Time for part 1: {:.2?}", start_time.elapsed());

    let lowest = seeds_part_2.par_iter().map( | seed_range | {
        let lowest: i64 = (0..seed_range.range).into_par_iter().map( | i | {
            let seed = seed_range.start + i;

            get_location_of_seed(&soil_mapping, &fertilizer_mapping, &water_mapping, &light_mapping, &temperature_mapping, &humidity_mapping, &location_mapping, seed)
        }).min().unwrap();

        let range = seed_range.range;
        println!("The seed range {range} has the lowest value {lowest}");
        lowest
    }).min().unwrap();

    println!("Part 2: The lowest location number is {lowest}.");
    println!("Time: {:.2?}", start_time.elapsed());
}

fn get_location_of_seed(soil_mapping: &RangeVec, fertilizer_mapping: &RangeVec, water_mapping: &RangeVec, light_mapping: &RangeVec, temperature_mapping: &RangeVec, humidity_mapping: &RangeVec, location_mapping: &RangeVec, seed: i64) -> i64 {
    let soil = soil_mapping.get(seed).unwrap();
    let fertilizer = fertilizer_mapping.get(soil).unwrap();
    let water = water_mapping.get(fertilizer).unwrap();
    let light = light_mapping.get(water).unwrap();
    let temperature = temperature_mapping.get(light).unwrap();
    let humidity = humidity_mapping.get(temperature).unwrap();
    let location = location_mapping.get(humidity).unwrap();
    location
}


fn get_range_seeds(input: &Vec<i64>) -> Vec<RangeSeed> {
    let mut seeds = Vec::new();

    for i in 0..input.len() {
        if i % 2 == 0 {
            seeds.push(RangeSeed { start: input[i], range: input[i + 1]})
        }
    }

    seeds
}

fn get_seeds(input: &String) -> Vec<i64> {
    input[7..].split(' ').map(|s: &str| s.parse::<i64>().unwrap()).collect()
}

fn get_mapping(input: &String) -> RangeVec {
    let mut map: RangeVec = RangeVec::new();

    for (index, line) in input.split('\n').enumerate() {
        if index == 0 { continue }  // Skip header
        let mapping: Vec<i64> = line.split(' ').map(|s: &str| s.parse::<i64>().unwrap()).collect();

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
