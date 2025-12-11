use std::{collections::BinaryHeap, time::Instant};

use helpers::*;

mod tests;

fn main() {
    let input = get_input(&get_path_from_arg());

    let start_time = Instant::now();
    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");
    let part_1_elapsed = start_time.elapsed();
    println!("Part 1 time: {:.2?}", part_1_elapsed);

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
    let part_2_elapsed = start_time.elapsed() - part_1_elapsed;
    println!("Part 2 time: {:.2?}", part_2_elapsed);

    println!("Total time: {:.2?}", start_time.elapsed());
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
    let edges = get_edges(&points);

    points
        .iter()
        .enumerate()
        .map(|(index, &point1)| {
            points
                .iter()
                .skip(index + 1)
                .map(|&point2| {
                    if edges.iter().any(|&edge| edge.intersects(&point1, &point2)) {
                        -1
                    } else {
                        calculate_area(&point1, &point2)
                    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    start: Point,
    end: Point,
}

impl Edge {
    fn intersects(&self, point1: &Point, point2: &Point) -> bool {
        // Get inner part bounds of rectangle
        let rect_x_min = point1.x.min(point2.x) + 1;
        let rect_x_max = point1.x.max(point2.x) - 1;
        let rect_y_min = point1.y.min(point2.y) + 1;
        let rect_y_max = point1.y.max(point2.y) - 1;

        let line_x_min = self.start.x.min(self.end.x);
        let line_x_max = self.start.x.max(self.end.x);
        let line_y_min = self.start.y.min(self.end.y);
        let line_y_max = self.start.y.max(self.end.y);

        line_y_max >= rect_y_min
            && line_y_min <= rect_y_max
            && line_x_max >= rect_x_min
            && line_x_min <= rect_x_max
    }
}

fn get_edges(points: &[Point]) -> Vec<Edge> {
    points
        .iter()
        .enumerate()
        .map(|(index, &point)| Edge {
            start: point,
            end: if index == points.len() - 1 {
                points[0]
            } else {
                points[index + 1]
            },
        })
        .collect()
}
