use memoize::memoize;
use crate::Condition::{Damaged, Operational, Unknown};

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
    let field = parse_field(input);

    let mut arrangement_sum = 0;

    for row in field {
        arrangement_sum += row_analyze(row);
    }

    arrangement_sum
}



fn part_2(input: &Vec<String>) -> i32 {
    let field = parse_folded_field(input);

    let mut arrangement_sum = 0;

    for row in field {
        arrangement_sum += row_analyze(row);
    }

    arrangement_sum
}


#[derive(PartialEq, Clone, Debug, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Row {
    springs: Vec<Condition>,
    damaged_spring_groups: Vec<i32>,
}

impl Condition {
    fn mapping(c: char) -> Self {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("Bad condition character!"),
        }
    }
}

fn parse_field(input: &Vec<String>) -> Vec<Row> {
    let mut rows = Vec::new();

    for line in input {
        rows.push(parse_row(line));
    }

    rows
}

fn parse_folded_field(input: &Vec<String>) -> Vec<Row> {
    let mut rows = Vec::new();

    for line in input {
        rows.push(parse_folded_row(line));
    }

    rows
}

fn parse_row(input: &String) -> Row {
    let parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
    let mut springs = Vec::new();

    for c in parts[0].chars() {
        springs.push(Condition::mapping(c));
    }

    let damaged_spring_groups: Vec<i32> = parts[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();

    Row { springs, damaged_spring_groups }
}

fn parse_folded_row(input: &String) -> Row {
    let parts: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
    let mut springs = Vec::new();
    let mut unfolded_damaged_spring_groups: Vec<i32> = Vec::new();
    let damaged_spring_groups: Vec<i32> = parts[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();

    for i in 0..5 {
        for c in parts[0].chars() {
            springs.push(Condition::mapping(c));
        }
        if i != 4 { springs.push(Unknown); }

        for group in &damaged_spring_groups {
            unfolded_damaged_spring_groups.push(*group);
        }
    }

    Row { springs, damaged_spring_groups: unfolded_damaged_spring_groups}
}

#[memoize]
fn row_analyze(row: Row) -> i32 {
    let mut row = row.clone();

    // No springs left
    if row.springs.len() == 0 {
        // Check if we used up all the damaged spring groups as well
        return if row.damaged_spring_groups.len() != 0 { 0 } else { 1 }
    }

    // No damaged blocks left
    if row.damaged_spring_groups.len() == 0 {
        // See if none of the springs are damaged
        for spring in &row.springs {
            if *spring == Damaged {
                return 0;
            }
        }
        return 1;
    }

    // Match with the first spring
    match row.springs[0] {
        Operational => {
            row.springs.remove(0);
            row_analyze(row)
        },
        Unknown => {
            row.springs[0] = Operational;
            let mut result = row_analyze(row.clone());
            row.springs[0] = Damaged;
            result += row_analyze(row);
            result
        },
        Damaged => {
            // Check if we have enough springs left
            if row.springs.len() < row.damaged_spring_groups[0].try_into().unwrap() {
                return 0;
            }

            // Check contiguous block
            for spring_index in 0..row.damaged_spring_groups[0] {
                if row.springs[spring_index as usize] == Operational { return 0; }
            }

            // If the next index after the block exists, make sure it's not damaged
            if ((row.damaged_spring_groups[0] + 1) as usize) <= row.springs.len() {
                if row.springs[row.damaged_spring_groups[0] as usize] == Damaged { return 0; }
                // If the next index is unknown, set it to operational (must be)
                else if row.springs[row.damaged_spring_groups[0] as usize] == Unknown {
                    row.springs[row.damaged_spring_groups[0] as usize] = Operational;
                }
            }

            // Remove the block and springs
            row.springs.drain(0..row.damaged_spring_groups[0] as usize);
            row.damaged_spring_groups.remove(0);

            row_analyze(row)
        },
    }
}