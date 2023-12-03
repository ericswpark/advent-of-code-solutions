use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};


fn main() {
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
    let input: Vec<Vec<char>> = fs::read_to_string(path)
        .expect("Couldn't read input file").split('\n')
        .map(|s: &str| s.chars().collect())
        .collect();

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
                let char_index: i32 = char_index as i32 - 1; // Number end was one character before
                let line_index: i32 = line_index as i32;

                // Check previous line (if it exists)
                if line_index - 1 >= 0 {
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

    println!("The part number sum is {sum}.");

}

fn check_char(is_part: &mut bool, target: char) {
    if !target.is_numeric() && target != '.' {
        *is_part = true
    }
}