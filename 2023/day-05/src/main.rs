use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::collections::BTreeMap;

fn main() {
    let input = get_input();

    let seeds = get_seeds(&input[0]);
    let soil_mapping = get_mapping(&input[1]);
    let fertilizer_mapping = get_mapping(&input[2]);
    let water_mapping = get_mapping(&input[3]);
    let light_mapping = get_mapping(&input[4]);
    let temperature_mapping = get_mapping(&input[5]);
    let humidity_mapping = get_mapping(&input[6]);
    let location_mapping = get_mapping(&input[7]);

    let mut lowest = i32::MAX;

    for seed in seeds {
        let soil = match soil_mapping.get(&seed) {
            None => { seed }
            Some(i) => { *i }
        };

        let fertilizer = match fertilizer_mapping.get(&soil) {
            None => { soil }
            Some(i) => { *i }
        };

        let water = match water_mapping.get(&fertilizer) {
            None => { fertilizer }
            Some(i) => { *i }
        };

        let light = match light_mapping.get(&water) {
            None => { water }
            Some(i) => { *i }
        };

        let temperature = match temperature_mapping.get(&light) {
            None => { light }
            Some(i) => { *i }
        };

        let humidity = match humidity_mapping.get(&temperature) {
            None => { temperature }
            Some(i) => { *i }
        };

        let location = match location_mapping.get(&humidity) {
            None => { humidity }
            Some(i) => { *i }
        };

        if location < lowest { lowest = location }
    }

    println!("The lowest location number is {lowest}.")
}

fn get_seeds(input: &String) -> Vec<i32> {
    input[7..].split(' ').map(|s: &str| s.parse::<i32>().unwrap()).collect()
}

fn get_mapping(input: &String) -> BTreeMap<i32, i32> {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();

    for (index, line) in input.split('\n').enumerate() {
        if index == 0 { continue }  // Skip header
        let mapping: Vec<i32> = line.split(' ').map(|s: &str| s.parse::<i32>().unwrap()).collect();

        let dest_range_start = mapping[0];
        let source_range_start = mapping[1];
        let range_length = mapping[2];

        for i in 0..range_length {
            map.insert(source_range_start + i, dest_range_start + i);
        }
    }

    map
}

fn get_input() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();

    let mut path: &mut String = &mut String::new();

    if args.len() < 2 {
        print!("Enter path to file: ");
        stdout().flush().expect("Cannot flush buffer");

        stdin().read_line(path).expect("Cannot process input");
        if let Some('\n')=path.chars().next_back() {
            path.pop();
        }
        if let Some('\r')=path.chars().next_back() {
            path.pop();
        }
    } else {
        path = &mut args[1];
    }

    fs::read_to_string(path)
        .expect("Couldn't read input file").split("\n\n").map(|s| s.to_string()).collect()
}
