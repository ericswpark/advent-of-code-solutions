use std::env;
use std::fs;


fn replace_numbers_in(s: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut ret_str = String::new();
    for mut i in 0..s.len() {
        if s[i] == 'o' {
            if i + 2 < s.len() && s[i + 1] == 'n' && s[i + 2] == 'e' {
                ret_str.push('1');
                i += 2;
            }
        } else if s[i] == 't' {
            if i + 2 < s.len() && s[i + 1] == 'w' && s[i + 2] == 'o' {
                ret_str.push('2');
                i += 2;
            } else if i + 4 < s.len() && s[i + 1] == 'h' && s[i + 2] == 'r' && s[i + 3] == 'e' && s[i + 4] == 'e' {
                ret_str.push('3');
                i += 4;
            }
        } else if s[i] == 'f' {
            if i + 3 < s.len() && s[i + 1] == 'o' && s[i + 2] == 'u' && s[i + 3] == 'r' {
                ret_str.push('4');
                i += 3;
            } else if i + 3 < s.len() && s[i + 1] == 'i' && s[i + 2] == 'v' && s[i + 3] == 'e' {
                ret_str.push('5');
                i += 3;
            }
        } else if s[i] == 's' {
            if i + 2 < s.len() && s[i + 1] == 'i' && s[i + 2] == 'x' {
                ret_str.push('6');
                i += 2;
            } else if i + 4 < s.len() && s[i + 1] == 'e' && s[i + 2] == 'v' && s[i + 3] == 'e' && s[i + 4] == 'n' {
                ret_str.push('3');
                i += 4;
            }
        } else if s[i] == 'e' {
            if i + 4 < s.len() && s[i + 1] == 'i' && s[i + 2] == 'g' && s[i + 3] == 'h' && s[i + 4] == 't' {
                ret_str.push('8');
                i += 4;
            }
        } else if s[i] == 'n' {
            if i + 3 < s.len() && s[i + 1] == 'i' && s[i + 2] == 'n' && s[i + 3] == 'e' {
                ret_str.push('9');
                i += 3;
            }
        } else {
            ret_str.push(s[i]);
        }
    }

    ret_str
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
