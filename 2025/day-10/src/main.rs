use helpers::*;

use itertools::Itertools;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let machines = parse_machines(input);
    machines.iter().map(get_least_presses).sum()
}

fn part_2(input: &[String]) -> i64 {
    todo!();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    indicators: Indicators,
    button_wirings: Vec<ButtonWiring>,
    joltage_requirements: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Indicators(Vec<bool>);

impl FromIterator<bool> for Indicators {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ButtonWiring(Vec<usize>);

impl FromIterator<usize> for ButtonWiring {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a ButtonWiring {
    type Item = usize;
    type IntoIter = ButtonWiringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ButtonWiringIterator {
            button_wiring: self,
            index: 0,
        }
    }
}

struct ButtonWiringIterator<'a> {
    button_wiring: &'a ButtonWiring,
    index: usize,
}

impl<'a> Iterator for ButtonWiringIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.button_wiring.0.len() {
            let button = self.button_wiring.0[self.index];
            self.index += 1;
            Some(button)
        } else {
            None
        }
    }
}

fn parse_machines(input: &[String]) -> Vec<Machine> {
    input.iter().map(parse_machine).collect()
}

fn parse_machine(line: &String) -> Machine {
    let mut parts = line.split_whitespace().peekable();

    // Get indicators
    let indicators: Indicators = parts
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| match c {
            '[' | ']' => None,
            '.' => Some(false),
            '#' => Some(true),
            _ => unreachable!("Invalid indicator character"),
        })
        .collect();

    let mut button_wirings = Vec::new();

    while parts.peek().unwrap().starts_with('(') {
        let button_wiring_str = parts.next().unwrap();
        let button_wiring: ButtonWiring = button_wiring_str[1..button_wiring_str.len() - 1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        button_wirings.push(button_wiring);
    }

    let joltage_requirement_str = parts.next().unwrap();
    let joltage_requirements = joltage_requirement_str[1..joltage_requirement_str.len() - 1]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Machine {
        indicators,
        button_wirings,
        joltage_requirements,
    }
}

fn get_least_presses(machine: &Machine) -> i64 {
    machine
        .button_wirings
        .iter()
        .powerset()
        .flat_map(|method| {
            let mut current_indicators = vec![false; machine.indicators.0.len()];

            for &wiring in &method {
                for button in wiring {
                    current_indicators[button] = !current_indicators[button];
                }
            }

            if machine.indicators.0 == current_indicators {
                Some(method.len() as i64)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}
