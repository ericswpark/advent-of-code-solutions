mod enums;
pub mod structs;

use crate::structs::DigStep;

pub fn parse_instructions(input: &Vec<String>) -> Vec<DigStep> {
    let mut instructions: Vec<DigStep> = Vec::new();

    for line in input {
        instructions.push(line.parse().unwrap());
    }

    instructions
}