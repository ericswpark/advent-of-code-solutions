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
    let histories: Vec<Vec<i32>> = get_sequence(input);

    let mut extrapolated_sum = 0;

    for history in histories {
        extrapolated_sum += get_extrapolated_number(&history);
    }

    extrapolated_sum
}

fn part_2(input: &Vec<String>) -> i64 {
    // TODO: implement
    0
}

fn get_sequence(input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut histories = Vec::new();
    for line in input {
        // Construct new history item
        let history: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap() ).collect();
        histories.push(history);
    }

    histories
}

fn get_extrapolated_number(input: &Vec<i32>) -> i32 {
    // If all the input numbers are zero, return 0
    let mut is_all_zeros = true;
    for &num in input {
        if num != 0 { is_all_zeros = false }
    }
    if is_all_zeros { return 0 }

    // Otherwise, create a new vector with the differences
    let mut differences: Vec<i32> = Vec::new();

    for i in 1..input.len() {
        differences.push(input[i] - input[i - 1]);
    }

    input[input.len() - 1] + get_extrapolated_number(&differences)
}