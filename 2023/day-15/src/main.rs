mod custom_helper;
mod tests;

use custom_helper::get_input;
use helpers::get_path_from_arg;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}


fn part_1(input: &Vec<String>) -> i64 {
    let mut sum = 0;

    for step in input {
        sum += hash_algo(step) as i64;
    }

    sum
}

#[derive(Clone)]
struct LensBox {
    lenses: Vec<Lens>
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn part_2(input: &Vec<String>) -> i64 {
    // Create 0~255 lens boxes
    let mut lens_boxes: Vec<LensBox> = vec![LensBox { lenses: Vec::new() }; 256];

    for step in input {
        let (label, operation, focal_length) = parse_step(step);

        let box_num = hash_algo(&label) as usize;

        match operation {
            Operation::Set => {
                let mut found_lens = false;
                for lens in &mut lens_boxes[box_num].lenses {
                    if lens.label == label {
                        found_lens = true;
                        lens.focal_length = focal_length;
                        break;
                    }
                }
                if !found_lens {
                    lens_boxes[box_num].lenses.push(Lens { label, focal_length })
                }
            }
            Operation::Remove => {
                for (index, lens) in lens_boxes[box_num].lenses.iter().enumerate() {
                    if lens.label == label {
                        lens_boxes[box_num].lenses.remove(index);
                        break;
                    }
                }
            }
        }
    }

    let mut focusing_power_sum: i64 = 0;

    for (index, lens_box) in lens_boxes.iter().enumerate() {
        focusing_power_sum += calculate_focus_power(lens_box, index) as i64;
    }

    focusing_power_sum
}

fn calculate_focus_power(lens_box: &LensBox, box_index: usize) -> i32 {
    let mut sum: i32 = 0;
    for (lens_index, lens) in lens_box.lenses.iter().enumerate() {
        sum += ((box_index + 1) * (lens_index + 1) * lens.focal_length as usize) as i32;
    }
    sum
}

enum Operation {
    Set,
    Remove,
}

fn parse_step(input: &String) -> (String, Operation, u8) {
    return if !input.chars().nth(input.len() - 1).unwrap().is_numeric() {
        ((&input[0..input.len() - 1]).to_string(), Operation::Remove, 0)
    } else {
        let focal_length: u8 = input.chars().nth(input.len() - 1).unwrap().to_digit(10).unwrap() as u8;
        ((&input[0..input.len() - 2]).to_string(), Operation::Set, focal_length)
    }
}


fn hash_algo(input: &String) -> u32 {
    let mut value = 0;

    for c in input.chars() {
        // You can directly get the ASCII value like this in Rust!
        value += c as u32;
        value *= 17;
        value %= 256;
    }

    value
}