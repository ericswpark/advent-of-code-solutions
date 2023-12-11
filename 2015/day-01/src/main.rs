mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}


fn part_1(input: &Vec<String>) -> i32 {
    let input = &input[0];

    get_level(input)
}



fn part_2(input: &Vec<String>) -> usize {
    let input = &input[0];

    get_basement_position(input) + 1
}

fn get_level(input: &String) -> i32{
    let mut level = 0;
    for c in input.chars() {
        if c == '(' { level += 1 }
        else if c == ')' { level -= 1 }
    }
    level
}

fn get_basement_position(input: &String) -> usize {
    let mut level = 0;
    for (index, c) in input.chars().enumerate() {
        if c == '(' { level += 1 }
        else if c == ')' { level -= 1 }

        if level < 0 { return index }
    }
    panic!("No basement level")
}