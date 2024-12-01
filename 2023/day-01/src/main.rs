
use helpers::*;

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
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &[String]) -> i32 {
    get_sum(input, false)
}

fn part_2(input: &[String]) -> i32 {
    get_sum(input, true)
}


fn get_sum(input: &[String], replace_num: bool) -> i32 {
    let mut sum = 0;

    for line in input {
        let mut first: i32 = -1;
        let mut last: i32= -1;
        let line: String = if !replace_num {line.to_string()} else {replace_numbers_in(line.to_string())};

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

    sum
}