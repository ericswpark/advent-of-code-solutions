use std::time::Instant;

mod helpers;
mod tests;


fn main() {
    let start_time = Instant::now();
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");

    let elapsed_time = start_time.elapsed();
    println!("Time: {:.2?}", elapsed_time);
}

fn part_1(input: &Vec<String>) -> i64 {
    todo!()
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}
