use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashSet},
    ops::Range,
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
    let points = parse_points(input);
    let seen_points: HashSet<Point> = HashSet::from_iter(points.iter().cloned());

    points
        .iter()
        .enumerate()
        .map(|(index, &point)| {
            points
                .iter()
                .skip(index + 1)
                .map(|&other_point| {
                    let pair1 = Point {
                        x: point.x,
                        y: other_point.y,
                    };
                    let pair2 = Point {
                        x: other_point.x,
                        y: point.y,
                    };
                    // Find connecting point
                    if seen_points.contains(&pair1) {
                        // See if there is some point further away than the other pair
                        if (pair1.x > pair2.x
                            && pair1.y > pair2.y
                            && points.iter().any(|&p| p.x <= pair2.x && p.y <= pair2.y))
                            || (pair1.x > pair2.x
                                && pair1.y < pair2.y
                                && points.iter().any(|&p| p.x <= pair2.x && p.y >= pair2.y))
                            || (pair1.x < pair2.x
                                && pair1.y > pair2.y
                                && points.iter().any(|&p| p.x >= pair2.x && p.y <= pair2.y))
                            || (pair1.x < pair2.x
                                && pair1.y < pair2.y
                                && points.iter().any(|&p| p.x >= pair2.x && p.y >= pair2.y))
                        {
                            // Check if there are any points within the rectangle formed
                            let x_range = pair1.x.min(pair2.x) + 1..pair1.x.max(pair2.x);
                            let y_range = pair1.y.min(pair2.y) + 1..pair1.y.max(pair2.y);
                            if points
                                .iter()
                                .any(|&check_point| within_range(&x_range, &y_range, &check_point))
                            {
                                return -1;
                            }

                            // This rectangle can be formed
                            return calculate_area(&point, &other_point);
                        }
                    } else if seen_points.contains(&pair2) {
                        // See if there is some point further away than the other pair
                        if (pair1.x > pair2.x
                            && pair1.y > pair2.y
                            && points.iter().any(|&p| p.x >= pair1.x && p.y >= pair1.y))
                            || (pair1.x > pair2.x
                                && pair1.y < pair2.y
                                && points.iter().any(|&p| p.x >= pair1.x && p.y <= pair1.y))
                            || (pair1.x < pair2.x
                                && pair1.y > pair2.y
                                && points.iter().any(|&p| p.x <= pair1.x && p.y >= pair1.y))
                            || (pair1.x < pair2.x
                                && pair1.y < pair2.y
                                && points.iter().any(|&p| p.x <= pair1.x && p.y <= pair1.y))
                        {
                            // Check if there are any points within the rectangle formed
                            let x_range = pair1.x.min(pair2.x) + 1..pair1.x.max(pair2.x);
                            let y_range = pair1.y.min(pair2.y) + 1..pair1.y.max(pair2.y);
                            if points
                                .iter()
                                .any(|&check_point| within_range(&x_range, &y_range, &check_point))
                            {
                                return -1;
                            }

                            // This rectangle can be formed
                            return calculate_area(&point, &other_point);
                        }
                    }
                    -1
                })
                .max()
                .unwrap_or(-1)
        })
        .max()
        .unwrap()
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

fn within_range(
    x_range: &std::ops::Range<i64>,
    y_range: &std::ops::Range<i64>,
    point: &Point,
) -> bool {
    x_range.contains(&point.x) && y_range.contains(&point.y)
}
