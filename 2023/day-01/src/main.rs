use std::env;
use std::fs;


fn replace_numbers_in(s: String) -> String {
   let mut s = s.replace("one", "o1e");
    s = s.replace("two", "t2o");
    s = s.replace("three", "t3e");
    s = s.replace("four", "f4r");
    s = s.replace("five", "f5e");
    s = s.replace("six", "s6x");
    s = s.replace("seven", "s7n");
    s = s.replace("eight", "e8t");
    s.replace("nine", "n9e")
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
