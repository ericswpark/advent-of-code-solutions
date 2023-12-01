use std::env;
use std::fs;
use phf::phf_map;

static NUMBERS: phf::Map<&'static str, &'static str> = phf_map! {
    "one" => "o1e",
    "two" => "t2o",
    "three" => "t3e",
    "four" => "f4r",
    "five" => "f5e",
    "six" => "s6x",
    "seven" => "s7n",
    "eight" => "e8t",
    "nine" => "n9e",
};

fn replace_numbers_in(s: String) -> String {
    let mut s = s;
    for number in NUMBERS.keys() {
        s = s.replace(number, NUMBERS[number])
    }

    s
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments given.");
        return;
    }

    let part: u32 = args[1].to_string().parse().unwrap();
    let path: &String = &args[2];
    let input: String = fs::read_to_string(path).expect("Couldn't read input file");

    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut first: i32 = -1;
        let mut last: i32= -1;
        let line: String = if part == 1 {line.to_string()} else {replace_numbers_in(line.to_string())};

        println!("Got line {line}");
        // Iterate over each character
        for c in line.chars() {
            if c.is_numeric() {
                if first == -1 {
                    first = c.to_digit(10).unwrap() as i32;
                }

                last = c.to_digit(10).unwrap() as i32;
            }
        }
        println!("We got the numbers {first} and {last}");

        // Add final number to sum
        sum += first * 10 + last;
    }

    println!("Sum is {sum}");
}
