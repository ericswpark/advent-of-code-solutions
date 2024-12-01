use std::fs;

pub fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Couldn't read input file").split(',').map(|s| s.to_string()).collect()
}