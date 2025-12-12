use helpers::*;

mod structs;
mod tests;

use structs::*;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let puzzle_data = parse_puzzledata(input);

    puzzle_data
        .regions
        .iter()
        .map(|region| easy_fit_check(region, &puzzle_data.shapes))
        .filter(|&result| result)
        .count() as i64
}

fn part_2(_input: &[String]) -> i64 {
    println!("No part 2 for last day");
    -1
}

fn parse_puzzledata(input: &[String]) -> PuzzleData {
    let mut iterator = input.iter();

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    while let Some(line) = iterator.next() {
        let parts = line.split(":").collect::<Vec<_>>();

        if parts[1] == "" {
            // Next couple of lines will be a shape definition
            let shape = iterator
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("Invalid present shape character"),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>();
            shapes.push(Shape(shape));
        } else {
            // The second part contains the region definition
            let dimensions: (usize, usize) = parts[0]
                .split_once('x')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let requirements: Vec<usize> = parts[1]
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push(Region {
                dimensions,
                requirements,
            });
        }
    }

    PuzzleData { shapes, regions }
}

fn easy_fit_check(region: &Region, shapes: &[Shape]) -> bool {
    // Check if there's enough area to even service all shapes assuming we can
    // split shapes
    let region_area = region.dimensions.0 * region.dimensions.1;
    let min_shape_required_area = region
        .requirements
        .iter()
        .enumerate()
        .map(|(i, &req)| {
            let shape = &shapes[i];
            shape.area() * req
        })
        .sum::<usize>();

    if region_area < min_shape_required_area {
        return false;
    }

    // Check if each 3x3 entry can fit in the region
    let region_allowed_entries = region.dimensions.0 / 3 * region.dimensions.1 / 3;
    let total_entries = region.requirements.iter().sum::<usize>();

    if total_entries < region_allowed_entries {
        return true;
    }

    true
}
