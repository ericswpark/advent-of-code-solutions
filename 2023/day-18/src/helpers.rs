use std::io::{stdin, stdout, Write};
use std::{env, fs};

pub fn get_path_from_arg() -> String {
    let mut args: Vec<String> = env::args().collect();

    let mut path: &mut String = &mut String::new();

    if args.len() < 2 {
        print!("Enter path to file: ");
        stdout().flush().expect("Cannot flush buffer");

        stdin().read_line(path).expect("Cannot process input");
        if let Some('\n') = path.chars().next_back() {
            path.pop();
        }
        if let Some('\r') = path.chars().next_back() {
            path.pop();
        }
    } else {
        path = &mut args[1];
    }

    path.to_owned()
}

pub fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Couldn't read input file")
        .split('\n')
        .map(|s| s.to_string())
        .collect()
}