mod tests;

use helpers::*;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let mut total = 0;

    for line in input {
        let Dimension((l, w, h)) = parse_dimensions(line);
        let areas = [l * w, w * h, h * l];
        println!("{} {} {}", l, w, h);
        let area: i64 = areas.iter().map(|x| x * 2).sum();
        let slack = areas.iter().min().unwrap();

        total += area + slack;
    }

    total
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone, Copy)]
struct Dimension((i64, i64, i64));

fn parse_dimensions(input: &str) -> Dimension {
    let mut parts = input.split('x');
    let l = parts.next().unwrap().parse().unwrap();
    let w = parts.next().unwrap().parse().unwrap();
    let h = parts.next().unwrap().parse().unwrap();
    Dimension((l, w, h))
}
