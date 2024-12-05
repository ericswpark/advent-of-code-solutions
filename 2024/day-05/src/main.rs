use std::collections::HashMap;
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

fn part_1(input: &Vec<String>) -> i64 {
    let (page_ordering_rules, update_page_numbers) = parse_input(input);
    let correct_updates = get_correct_updates(&page_ordering_rules, &update_page_numbers);

    println!("{:?}", correct_updates);
    let mut sum = 0;
    for update in correct_updates {
        sum += get_middle(&update);
    }
    sum
}

fn part_2(input: &Vec<String>) -> i64 {
    todo!()
}


fn parse_input(input: &[String]) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut page_ordering_rules: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut update_page_numbers = Vec::new();

    let mut is_on_next_section = false;
    for line in input {
        if line == "" {
            is_on_next_section = true;
            continue;
        }

        if !is_on_next_section {
            let [a, b] = line.split('|').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
            if let Some(next_pages) = page_ordering_rules.get_mut(&a) {
                next_pages.push(b);
            } else {
                page_ordering_rules.insert(a, vec![b]);
            }
            continue;
        }

        let current_update_page_numbers = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        update_page_numbers.push(current_update_page_numbers);
    }

    (page_ordering_rules, update_page_numbers)
}

fn get_correct_updates(page_ordering_rules: &HashMap<i64, Vec<i64>>, update_page_numbers: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut correct_updates = Vec::new();

    for update in update_page_numbers {
        if update_is_correct(page_ordering_rules, update) {
            correct_updates.push(update.clone());
        }
    }

    correct_updates
}

fn update_is_correct(page_ordering_rules: &HashMap<i64, Vec<i64>>, update: &[i64]) -> bool {
    for (index, num) in update.iter().enumerate().rev() {
        if let Some(next_pages) = page_ordering_rules.get(num) {
            for f_index in 0..index {
                let prev_page = update[f_index];
                if next_pages.contains(&prev_page) {
                    return false;
                }
            }
        }
        continue;
    }

    true
}

fn get_middle(update: &[i64]) -> i64 {
    update[update.len() / 2]
}