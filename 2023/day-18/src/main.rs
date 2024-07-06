mod helpers;
mod tests;

use std::time::Instant;

use day_18::parse_instructions;


fn main() {
    let program_start_time = Instant::now();
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_1_time = program_start_time.elapsed();
    let part_2_start_time = Instant::now();

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let part_2_time = part_2_start_time.elapsed();
    let total_time = program_start_time.elapsed();

    println!("Part 1: {:.2?}", part_1_time);
    println!("Part 2: {:.2?}", part_2_time);
    println!("Total: {:.2?}", total_time);
}

fn part_1(input: &Vec<String>) -> i64 {
    let digging_instructions = parse_instructions(input);

    for instruction in digging_instructions {
        for iter in 0..instruction.repeat {
            println!("Would've dug {}, repeated {} times", instruction.direction, iter);
        }
    }

    0
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}

