use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() {
    let input = get_input();

    let sum = get_part_sum(&input);
    let gear_ratio_sum = get_gear_ratio_sum(&input);

    println!("The part number sum is {sum}.");
    println!("The gear ratio sum is {gear_ratio_sum}.");
}

fn get_input() -> Vec<Vec<char>> {
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
        .expect("Couldn't read input file").split('\n')
        .map(|s: &str| s.chars().collect())
        .collect()
}

fn get_gear_ratio_sum(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for (line_index, line) in input.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if *char == '*' {
                // Find two numbers adjacent
                let mut numbers: Vec<i32> = Vec::new();

                // Check previous line (if it exists)
                if line_index > 0 {
                    let numbers_from_target_line = get_number_from_line(&input[line_index - 1], char_index as i32);
                    match numbers_from_target_line {
                        None => {}
                        Some(val) => {
                            for n in val { numbers.push(n) }
                        }
                    }
                }

                if char_index > 0 && line[char_index - 1].is_numeric() {
                    numbers.push(get_whole_number(line, (char_index - 1) as i32));
                }

                if char_index + 1 < line.len() && line[char_index + 1].is_numeric() {
                    numbers.push(get_whole_number(line, (char_index + 1) as i32));
                }

                if line_index + 1 < input.len() {
                    let numbers_from_target_line = get_number_from_line(&input[line_index + 1], char_index as i32);
                    match numbers_from_target_line {
                        None => {}
                        Some(val) => {
                            for n in val { numbers.push(n) }
                        }
                    }
                }

                if numbers.len() == 2 {
                    // Multiply the two numbers and then add to sum
                    let gear_ratio = numbers.first().unwrap() * numbers.get(1).unwrap();
                    sum += gear_ratio;
                }
            } else {
                continue
            }
        }
    }

    sum
}

fn get_part_sum(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for (line_index, line) in input.iter().enumerate() {
        let mut parsed_num = String::new();
        for (char_index, char) in line.iter().enumerate() {
            if char.is_numeric() {
                parsed_num.push(*char);
            }

            if char_index + 1 == line.len() || !char.is_numeric() {
                if parsed_num.is_empty() { continue }
                // End of number, see if it is adjacent to a symbol
                let mut is_part = false;
                // Temporarily reassign indexes
                let char_index: i32 = if char.is_numeric() { char_index as i32 } else { char_index as i32 - 1 };
                let line_index: i32 = line_index as i32;

                // Check previous line (if it exists)
                if line_index > 0 {
                    // Check diagonal left
                    if char_index - parsed_num.len() as i32 >= 0 {
                        check_char(&mut is_part, input[line_index as usize - 1][char_index as usize - parsed_num.len()]);
                    }
                    // Check characters on top
                    for i in 0..parsed_num.len() {
                        check_char(&mut is_part, input[line_index as usize - 1][char_index as usize + 1 + i - parsed_num.len()]);
                    }
                    // Check diagonal right
                    if char_index + 1 < line.len() as i32 {
                        check_char(&mut is_part, input[line_index as usize - 1][char_index as usize + 1]);
                    }
                }
                // Check left of number (if it exists)
                if char_index - parsed_num.len() as i32 >= 0 {
                    check_char(&mut is_part, line[char_index as usize - parsed_num.len()]);
                }
                // Check right of number (if it exists)
                if char_index + 1 < line.len() as i32 {
                    check_char(&mut is_part, line[char_index as usize + 1]);
                }
                // Check next line (if it exists)
                if line_index + 1 < input.len() as i32 {
                    // Check diagonal left
                    if char_index - parsed_num.len() as i32 >= 0 {
                        check_char(&mut is_part, input[line_index as usize + 1][char_index as usize - parsed_num.len()]);
                    }
                    // Check characters below
                    for i in 0..parsed_num.len() {
                        check_char(&mut is_part, input[line_index as usize + 1][char_index as usize + 1 + i - parsed_num.len()]);
                    }
                    // Check diagonal right
                    if char_index + 1 < line.len() as i32 {
                        check_char(&mut is_part, input[line_index as usize + 1][char_index as usize + 1]);
                    }
                }

                if is_part {
                    sum += parsed_num.parse::<i32>().unwrap();
                }

                // Reset number
                parsed_num = String::new();
            }
        }
    }

    sum
}

fn check_char(is_part: &mut bool, target: char) {
    if !target.is_numeric() && target != '.' {
        *is_part = true
    }
}

fn get_number_from_line(line: &Vec<char>, index: i32) -> Option<Vec<i32>> {
    let mut numbers: Vec<i32> = Vec::new();

    if index > 0 && line[index as usize - 1].is_numeric() {
        numbers.push(get_whole_number(line, index - 1));
    } else if line[index as usize].is_numeric() {
        numbers.push(get_whole_number(line, index))
    } else if index + 1 < line.len() as i32 && line[index as usize + 1].is_numeric() {
        numbers.push(get_whole_number(line, index + 1));
    }

    // It's possible that there are two numbers, separated by a middle character
    if index > 0 && line[index as usize - 1].is_numeric() && !line[index as usize].is_numeric() && index + 1 < line.len() as i32 && line[index as usize + 1].is_numeric() {
        numbers.push(get_whole_number(line, index + 1));
    }

    if numbers.is_empty() { None } else { Some(numbers) }
}

fn get_whole_number(line: &Vec<char>, index: i32) -> i32 {
    let mut start_index = index;
    let mut end_index = index;

    while start_index > 0 && line[start_index as usize - 1].is_numeric() {
        start_index -= 1;
    }

    while end_index + 1 < line.len() as i32 && line[end_index as usize + 1].is_numeric() {
        end_index += 1;
    }

    let result_string: String = line[start_index as usize..=end_index as usize].iter().collect();

    result_string.parse::<i32>().unwrap()
}