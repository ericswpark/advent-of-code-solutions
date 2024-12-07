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
    let equations = get_equations(input);

    let mut sum = 0;
    for equation in equations {
        if is_solvable(&equation) {
            sum += equation.test_value;
        }
    }

    sum
}

fn part_2(input: &[String]) -> i64 {
    let equations = get_equations(input);

    let mut sum = 0;
    for equation in equations {
        if is_expanded_solvable(&equation) {
            sum += equation.test_value;
        }
    }

    sum
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn get_equations(input: &[String]) -> Vec<Equation> {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let numbers_part = parts.next().unwrap();

            let numbers = numbers_part
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn is_solvable(equation: &Equation) -> bool {
    return is_solvable_sub(
        equation.test_value,
        equation.numbers[0],
        &equation.numbers[1..],
    );
}

fn is_solvable_sub(total: i64, left_num: i64, remaining_numbers: &[i64]) -> bool {
    if remaining_numbers.len() <= 1 {
        return (total == left_num + remaining_numbers[0])
            || (total == left_num * remaining_numbers[0]);
    }
    return is_solvable_sub(
        total,
        left_num + remaining_numbers[0],
        &remaining_numbers[1..],
    ) || is_solvable_sub(
        total,
        left_num * remaining_numbers[0],
        &remaining_numbers[1..],
    );
}

fn is_expanded_solvable(equation: &Equation) -> bool {
    return is_expanded_solvable_sub(
        equation.test_value,
        equation.numbers[0],
        &equation.numbers[1..],
    );
}

fn is_expanded_solvable_sub(total: i64, left_num: i64, remaining_numbers: &[i64]) -> bool {
    let right_num = remaining_numbers[0];
    let concat_num = (left_num.to_string() + &right_num.to_string())
        .parse::<i64>()
        .unwrap();

    if remaining_numbers.len() <= 1 {
        return (total == left_num + right_num)
            || (total == left_num * right_num)
            || (total == concat_num);
    }
    return is_expanded_solvable_sub(total, left_num + right_num, &remaining_numbers[1..])
        || is_expanded_solvable_sub(total, left_num * right_num, &remaining_numbers[1..])
        || is_expanded_solvable_sub(total, concat_num, &remaining_numbers[1..]);
}
