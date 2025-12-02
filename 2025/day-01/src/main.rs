use std::time::Instant;

use helpers::*;

mod tests;

fn main() {
    let input = get_input(&get_path_from_arg());

    let start_time = Instant::now();
    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &[String]) -> i64 {
    let mut current = 50;
    let mut password = 0;

    for line in input {
        let direction = line.chars().next().unwrap();
        let distance = line[1..].parse::<i64>().unwrap();
        match direction {
            'L' => current += distance,
            'R' => current -= distance,
            _ => unreachable!("Shouldn't happen, direction is either L or R"),
        }

        // Loop around
        while current > 99 {
            current -= 100;
        }
        while current < 0 {
            current += 100;
        }

        if current == 0 {
            password += 1;
        }
    }

    password
}

fn part_2(input: &[String]) -> i64 {
    todo!()
}
