use std::{collections::HashSet, time::Instant};

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

fn construct_map(input: &[String]) -> Vec<Vec<bool>> {
    let mut map = Vec::with_capacity(input.len());

    for line in input {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => false,
                '@' => true,
                _ => unreachable!("Incorrect input"),
            })
            .collect();
        map.push(row);
    }

    map
}

fn part_1(input: &[String]) -> i64 {
    let map = construct_map(input);

    let mut count = 0;

    for (x, row) in map.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            // Skip empty cells
            if !cell {
                continue;
            }

            // Look at all neighbors
            let potential_rows = [x.saturating_sub(1), x, x + 1];
            let potential_columns = [y.saturating_sub(1), y, y + 1];
            let neighbor_positions = potential_rows
                .iter()
                .flat_map(|&nx| potential_columns.iter().map(move |&ny| (nx, ny)))
                .filter(|&(nx, ny)| {
                    // Filter out OOB and current coordinate
                    nx < map.len() && ny < map[nx].len() && (nx != x || ny != y)
                })
                .collect::<HashSet<_>>();

            let neighbor_count = neighbor_positions
                .iter()
                .map(|&(nx, ny)| map[nx][ny])
                .filter(|&cell| cell)
                .count();

            if neighbor_count < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}
