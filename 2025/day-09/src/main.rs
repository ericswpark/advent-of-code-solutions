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
    let points = parse_points(input);
    let seen_points: HashSet<Point> = HashSet::from_iter(points.iter().cloned());

    points
        .iter()
        .enumerate()
        .map(|(index, &point1)| {
            points
                .iter()
                .skip(index + 1)
                .map(|&point2| {
                    let point3 = Point {
                        x: point1.x,
                        y: point2.y,
                    };
                    let point4 = Point {
                        x: point2.x,
                        y: point1.y,
                    };
                    // Find connecting point
                    if seen_points.contains(&point3) {
                        // See if there is some point further away than the other pair
                        if (point3.x > point4.x
                            && point3.y > point4.y
                            && points.iter().any(|&p| p.x <= point4.x && p.y <= point4.y))
                            || (point3.x > point4.x
                                && point3.y < point4.y
                                && points.iter().any(|&p| p.x <= point4.x && p.y >= point4.y))
                            || (point3.x < point4.x
                                && point3.y > point4.y
                                && points.iter().any(|&p| p.x >= point4.x && p.y <= point4.y))
                            || (point3.x < point4.x
                                && point3.y < point4.y
                                && points.iter().any(|&p| p.x >= point4.x && p.y >= point4.y))
                        {
                            // Check if there are any points within the rectangle formed
                            let (x_range, y_range) = get_ranges(&point3, &point4);
                            if points
                                .iter()
                                .any(|&check_point| within_range(&x_range, &y_range, &check_point))
                            {
                                return -1;
                            }

                            // This rectangle can be formed
                            return calculate_area(&point1, &point2);
                        }
                    } else if seen_points.contains(&point4) {
                        // See if there is some point further away than the other pair
                        if (point3.x > point4.x
                            && point3.y > point4.y
                            && points.iter().any(|&p| p.x >= point3.x && p.y >= point3.y))
                            || (point3.x > point4.x
                                && point3.y < point4.y
                                && points.iter().any(|&p| p.x >= point3.x && p.y <= point3.y))
                            || (point3.x < point4.x
                                && point3.y > point4.y
                                && points.iter().any(|&p| p.x <= point3.x && p.y >= point3.y))
                            || (point3.x < point4.x
                                && point3.y < point4.y
                                && points.iter().any(|&p| p.x <= point3.x && p.y <= point3.y))
                        {
                            // Check if there are any points within the rectangle formed
                            let (x_range, y_range) = get_ranges(&point3, &point4);
                            if points
                                .iter()
                                .any(|&check_point| within_range(&x_range, &y_range, &check_point))
                            {
                                return -1;
                            }

                            // This rectangle can be formed
                            return calculate_area(&point1, &point2);
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

fn get_ranges(point1: &Point, point2: &Point) -> (std::ops::Range<i64>, std::ops::Range<i64>) {
    (
        point1.x.min(point2.x) + 1..point1.x.max(point2.x),
        point1.y.min(point2.y) + 1..point1.y.max(point2.y),
    )
}

fn within_range(
    x_range: &std::ops::Range<i64>,
    y_range: &std::ops::Range<i64>,
    point: &Point,
) -> bool {
    x_range.contains(&point.x) && y_range.contains(&point.y)
}
