mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

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



fn part_2(input: &Vec<String>) -> i64 {
    todo!()
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