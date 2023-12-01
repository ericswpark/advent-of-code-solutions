use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments given.");
        return;
    }

    let path: &String = &args[1];
    let input: String = fs::read_to_string(path).expect("Couldn't read input file");

    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut first: i32 = -1;
        let mut last: i32= -1;

        // Iterate over each character
        for c in line.chars() {
            if c.is_numeric() {
                if first == -1 {
                    first = c.to_digit(10).unwrap() as i32;
                }

                last = c.to_digit(10).unwrap() as i32;
            }
        }

        // Add final number to sum
        sum += first * 10 + last;
    }

    println!("Sum is {sum}");
}
