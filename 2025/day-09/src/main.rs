use std::{
    collections::{BinaryHeap, HashSet},
    time::Instant,
};

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
    let points = parse_points(input);

    // Create max-heap of area
    let mut max_heap = BinaryHeap::new();

    for (index, point) in points.iter().enumerate() {
        for other_point in points.iter().skip(index + 1) {
            let area = calculate_area(point, other_point);
            max_heap.push(area);
        }
    }

    max_heap.pop().unwrap()
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_points(input: &[String]) -> Vec<Point> {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Point { x, y }
        })
        .collect()
}

fn calculate_area(point1: &Point, point2: &Point) -> i64 {
    let dx = point1.x.abs_diff(point2.x);
    let dy = point1.y.abs_diff(point2.y);
    ((dx + 1) * (dy + 1)) as i64
}
