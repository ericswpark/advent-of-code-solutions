use std::io::{Write, stdin, stdout};
use std::{env, fs};

#[macro_export]
macro_rules! aoc_main {
    () => {
        fn main() {
            use std::time::Instant;

            let input = get_input(&get_path_from_arg());

            let start_time = Instant::now();

            let part_1_answer = part_1(&input);
            println!("Part 1 answer: {part_1_answer}");
            let part_1_elapsed = start_time.elapsed();
            println!("Part 1 time: {:.2?}", part_1_elapsed);

            let part_2_answer = part_2(&input);
            println!("Part 2 answer: {part_2_answer}");
            let part_2_elapsed = start_time.elapsed() - part_1_elapsed;
            println!("Part 2 time: {:.2?}", part_2_elapsed);

            println!("Total time: {:.2?}", start_time.elapsed());
        }
    };
}

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
        .map(|s| s.strip_suffix("\r").unwrap_or(s).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let result = get_input("test.txt");
        assert_eq!(result[0], String::from("Some sample text"));
    }
}
