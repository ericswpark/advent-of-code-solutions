use std::{
    collections::{BinaryHeap, HashSet},
    time::Instant,
};

use helpers::*;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

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
    // Hard-coded to 1,000, but sample input is different
    build_and_get_circuit_result(input, 1000)
}

fn part_2(input: &[String]) -> i64 {
    let boxes = read_junction_boxes(input);
    let mut distance_pairs = get_distance_pairs(&boxes);
    let mut circuits = QuickUnionUf::<UnionBySize>::new(boxes.len());

    let last_connecting_pair: Option<DistancePair>;
    loop {
        let min = distance_pairs.pop().unwrap();
        circuits.union(min.first, min.second);

        if circuits.get(min.first).size() == boxes.len() {
            // We've reached the box that connects everything together
            last_connecting_pair = Some(min);
            break;
        }
    }

    let first_box = boxes[last_connecting_pair.unwrap().first];
    let second_box = boxes[last_connecting_pair.unwrap().second];

    first_box.x * second_box.x
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

fn read_junction_boxes(input: &[String]) -> Vec<JunctionBox> {
    input
        .iter()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            JunctionBox {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct DistancePair {
    first: usize,
    second: usize,
    distance: i64,
}

impl Ord for DistancePair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order for min-heap
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for DistancePair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_distance_pairs(boxes: &[JunctionBox]) -> BinaryHeap<DistancePair> {
    let mut pairs = BinaryHeap::new();
    for (i, box1) in boxes.iter().enumerate() {
        for (j, box2) in boxes.iter().enumerate().skip(i + 1) {
            let distance =
                ((box1.x - box2.x).pow(2) + (box1.y - box2.y).pow(2) + (box1.z - box2.z).pow(2))
                    .isqrt();
            pairs.push(DistancePair {
                first: i,
                second: j,
                distance,
            });
        }
    }
    pairs
}

fn build_and_get_circuit_result(input: &[String], count: usize) -> i64 {
    let boxes = read_junction_boxes(input);
    let mut distance_pairs = get_distance_pairs(&boxes);
    let mut circuits = QuickUnionUf::<UnionBySize>::new(boxes.len());

    for _ in 0..count {
        let min = distance_pairs.pop().unwrap();
        circuits.union(min.first, min.second);
    }

    // Build binary heap to get maximum circuit sizes
    let mut max_heap = BinaryHeap::with_capacity(boxes.len());
    let mut seen_roots = HashSet::new();
    // This is probably inefficient but who cares
    (0..boxes.len()).for_each(|i| {
        let root = circuits.find(i);
        if seen_roots.contains(&root) {
            return;
        }
        seen_roots.insert(root);

        let size = circuits.get(i).size() as i64;
        max_heap.push(size);
    });

    let mut total = 1;

    for _ in 0..3 {
        let biggest_circuit_entry = max_heap.pop().unwrap();
        total *= biggest_circuit_entry as i64;
    }

    total
}
