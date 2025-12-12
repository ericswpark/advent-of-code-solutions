use helpers::*;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let puzzle_data = parse_puzzledata(input);

    let mut satisfied = 0;

    for region in puzzle_data.regions {
        let mut queue = Vec::new();

        // Put all shape indices in queue
        for (index, entry) in region.requirements.iter().enumerate() {
            for _ in 0..*entry {
                queue.push(index);
            }
        }

        // Generate 2D map of region
        let mut map = vec![vec![false; region.dimensions.1]; region.dimensions.0];

        // Until we run out of items in the queue, try stuffing them into the map
        while let Some(shape_index) = queue.pop() {
            let shape = &puzzle_data.shapes[shape_index];

            // Try placing shape on map
            todo!();

            // If shape does not fit, push back into queue and quit early
            todo!();
        }

        if queue.is_empty() {
            satisfied += 1;
        }
    }

    satisfied
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone)]
struct PuzzleData {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone)]
struct Shape(Vec<Vec<bool>>);

#[derive(Debug, Clone)]
struct Region {
    dimensions: (usize, usize),
    requirements: Vec<usize>,
}

fn parse_puzzledata(input: &[String]) -> PuzzleData {
    let mut iterator = input.iter();

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    if let Some(line) = iterator.next() {
        let parts = line.split(":").collect::<Vec<_>>();

        if parts.len() == 1 {
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
            let requirements: Vec<usize> =
                parts[1].split(',').map(|s| s.parse().unwrap()).collect();
            regions.push(Region {
                dimensions,
                requirements,
            });
        }
    }

    PuzzleData { shapes, regions }
}
