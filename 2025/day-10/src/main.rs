use helpers::*;

use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};
use itertools::Itertools;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let machines = parse_machines(input);
    machines.iter().map(get_least_presses_indicators).sum()
}

fn part_2(input: &[String]) -> i64 {
    parse_machines(input)
        .iter()
        .map(get_least_presses_joltage)
        .sum()
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
    input.iter().map(|s| parse_machine(s)).collect()
}

fn parse_machine(line: &str) -> Machine {
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

fn get_least_presses_indicators(machine: &Machine) -> i64 {
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

fn get_least_presses_joltage(machine: &Machine) -> i64 {
    let counter_count = machine.joltage_requirements.len();
    let button_count = machine.button_wirings.len();

    // Create variables for buttons
    variables! {vars: 0 <= x[button_count] (integer) ; }

    // Keep track of what button causes counter to go up
    let mut counter_changing_button_indices = vec![vec![]; counter_count];
    for (index, button_wiring) in machine.button_wirings.iter().enumerate() {
        button_wiring.into_iter().for_each(|button| {
            counter_changing_button_indices[button].push(index);
        });
    }

    let mut constraints = Vec::new();
    for (counter_index, button_indices) in counter_changing_button_indices.iter().enumerate() {
        let joltage = button_indices
            .iter()
            .map(|&index| x[index])
            .sum::<Expression>();
        constraints.push(constraint!(
            joltage == machine.joltage_requirements[counter_index] as f64
        ));
    }

    let solution = vars
        .minimise(x.iter().sum::<Expression>())
        .using(default_solver)
        .with_all(constraints)
        .solve()
        .unwrap();

    (0..button_count)
        .map(|index| solution.value(x[index]) as i64)
        .sum()
}
