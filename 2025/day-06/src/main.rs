use helpers::*;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let arrays: Vec<Vec<String>> = parse_to_arrays(input);

    let number_arrays = &arrays[0..arrays.len() - 1];
    let operations = arrays.last().unwrap();

    operations
        .iter()
        .enumerate()
        .map(
            |(index, operation)| match operation.chars().next().unwrap() {
                '+' => {
                    let result: i64 = number_arrays
                        .iter()
                        .map(|numbers| numbers[index].parse::<i64>().unwrap())
                        .sum();
                    result
                }
                '*' => {
                    let result: i64 = number_arrays
                        .iter()
                        .map(|numbers| numbers[index].parse::<i64>().unwrap())
                        .product();
                    result
                }
                _ => unreachable!("Unsupported operation"),
            },
        )
        .sum()
}

fn part_2(input: &[String]) -> i64 {
    let mut operations = Vec::new();

    // Go through the last line and figure out the indices for operations
    for (index, ch) in input.last().unwrap().chars().enumerate() {
        match ch {
            '+' => operations.push(Operation {
                operation_type: OperationType::Add,
                start_index: index,
            }),
            '*' => operations.push(Operation {
                operation_type: OperationType::Multiply,
                start_index: index,
            }),
            ' ' => continue,
            _ => unreachable!("Unsupported operation"),
        }
    }

    // For each operation, get the corresponding numbers and perform calculation
    operations
        .iter()
        .enumerate()
        .map(|(index, operation)| {
            let next_operation = operations.get(index + 1);
            let mut end_index = input.iter().map(|line| line.len()).max().unwrap();

            // If there is a next operation
            if let Some(next_operation) = next_operation {
                // Set the ending index to one right before
                end_index = next_operation.start_index - 1;
            }

            // Iterate backwards and grab numbers
            let numbers: Vec<i64> = (operation.start_index..end_index)
                .map(|index| {
                    // Get each full number and add to operations
                    input.iter().take(input.len() - 1).fold(0i64, |acc, line| {
                        let ch = line.chars().nth(index);
                        if let Some(ch) = ch
                            && ch != ' '
                        {
                            let number = ch.to_digit(10).unwrap() as i64;
                            return acc * 10 + number;
                        }
                        acc
                    })
                })
                .collect::<Vec<i64>>();

            println!("Numbers: {:?}", numbers);

            // Perform calculation based on operation type
            match operation.operation_type {
                OperationType::Add => numbers.iter().sum::<i64>(),
                OperationType::Multiply => numbers.iter().product(),
            }
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    operation_type: OperationType,
    start_index: usize,
}

#[derive(Debug, Clone, Copy)]
enum OperationType {
    Add,
    Multiply,
}

fn parse_to_arrays(input: &[String]) -> Vec<Vec<String>> {
    input.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}
