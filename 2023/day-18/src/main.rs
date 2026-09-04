mod digstep;
mod direction;
mod tests;

use digstep::DigStep;
use direction::Direction::*;
use helpers::*;

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &[String]) -> i64 {
    let digsteps = get_digsteps(input).expect("Failed to parse digsteps");

    let coordinates = get_coordinates(&digsteps);

    calculate_polygon_area(&coordinates) + (calculate_perimeter_length(&digsteps) / 2 + 1)
}

fn part_2(input: &[String]) -> i64 {
    let digsteps = get_digsteps(input).expect("Failed to parse digsteps");
    let digsteps = get_corrected_digsteps(&digsteps);

    let coordinates = get_coordinates(&digsteps);

    calculate_polygon_area(&coordinates) + (calculate_perimeter_length(&digsteps) / 2 + 1)
}

fn get_digsteps(input: &[String]) -> Result<Vec<DigStep>, String> {
    input.iter().map(|s| s.parse()).collect()
}

fn get_coordinates(digsteps: &[DigStep]) -> Vec<Coordinates> {
    let mut coords = vec![];
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for step in digsteps {
        match step.direction {
            Left => {
                x -= step.length as i64;
            }
            Right => {
                x += step.length as i64;
            }
            Up => {
                y += step.length as i64;
            }
            Down => {
                y -= step.length as i64;
            }
        }

        coords.push(Coordinates { x, y })
    }

    coords
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i64,
    y: i64,
}

fn calculate_polygon_area(coordinates: &[Coordinates]) -> i64 {
    let mut first: Option<&Coordinates> = None;
    let mut iterator = coordinates.iter().peekable();
    let mut area: i64 = 0;

    while let Some(coord) = iterator.next() {
        let next_coord = if iterator.peek().is_some() {
            *iterator.peek().unwrap()
        } else {
            first.unwrap()
        };
        area += coord.x * next_coord.y - next_coord.x * coord.y;

        if first.is_none() {
            first = Some(coord);
        }
    }

    area.abs() / 2
}

fn calculate_perimeter_length(digsteps: &[DigStep]) -> i64 {
    digsteps.iter().map(|d| d.length).sum::<usize>() as i64
}

fn get_corrected_digsteps(digsteps: &[DigStep]) -> Vec<DigStep> {
    let mut corrected_digsteps = vec![];

    for step in digsteps {
        let hex_length: String = step.color.chars().take(5).collect();
        let direction = step.color.chars().last().unwrap();

        let direction = match direction {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("Invalid direction in color part"),
        };

        corrected_digsteps.push(DigStep {
            direction,
            length: usize::from_str_radix(&hex_length, 16).expect("Invalid length in color part"),
            color: String::new(),
        })
    }

    corrected_digsteps
}
