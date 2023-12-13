use std::cmp;

mod helpers;
mod tests;

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}


fn part_1(input: &Vec<String>) -> i64 {
    let patterns = parse_patterns(input);

    let mut summary = 0;

    for pattern in patterns {
        if get_vertical_reflection(&pattern) != -1 {
            summary += get_vertical_reflection(&pattern);
        } else if get_horizontal_reflection(&pattern) != -1 {
            summary += 100 * get_horizontal_reflection(&pattern);
        }
    }

    summary
}



fn part_2(input: &Vec<String>) -> i64 {
    let patterns = parse_patterns(input);

    let mut summary = 0;

    for pattern in patterns {
        // Get the original values to compare against
        let original_vertical = get_vertical_reflection(&pattern);
        let original_horizontal = get_horizontal_reflection(&pattern);

        // Test different smudges
        'outer_loop: for row in 0..pattern.len() {
            for column in 0..pattern[row].len() {
                let mut new_pattern = pattern.clone();
                new_pattern[row][column].flip();

                let new_vertical = get_vertical_reflection(&new_pattern);
                let new_horizontal = get_horizontal_reflection(&new_pattern);

                if new_vertical != -1 && new_vertical != original_vertical {
                    summary += new_vertical;
                    break 'outer_loop;
                } else if new_horizontal != -1 && new_horizontal != original_horizontal {
                    summary += 100 * new_horizontal;
                    break 'outer_loop;
                }

            }
        }
    }

    summary
}


#[derive(PartialEq, Clone, Debug, Eq, Hash)]
enum Item {
    Rock,
    Ash
}

impl Item {
    fn mapping(c: char) -> Self {
        match c {
            '.' => Item::Ash,
            '#' => Item::Rock,
            _ => panic!("Bad condition character!"),
        }
    }

    fn flip(&mut self) {
        if *self == Item::Ash {
            *self = Item::Rock;
        } else {
            *self = Item::Ash;
        }
    }
}

fn parse_patterns(input: &Vec<String>) -> Vec<Vec<Vec<Item>>> {
    let mut patterns = Vec::new();

    for line in input {
        let mut pattern = Vec::new();

        let rows: Vec<String> = line.split('\n').map(|s| s.to_string()).collect();

        for raw_row in rows {
            let row = parse_row(&raw_row);
            pattern.push(row);
        }

        patterns.push(pattern)
    }

    patterns
}

fn parse_row(input: &String) -> Vec<Item> {
    let mut row = Vec::new();
    for c in input.chars() {
        row.push(Item::mapping(c));
    }

    row
}

fn get_vertical_reflection(pattern: &Vec<Vec<Item>>) -> i64 {
    for column in 0..pattern[0].len() - 1 {
        // Start comparing
        // Get number of columns we need to compare
        let max_compare_column_length = cmp::min(column + 1, pattern[0].len() - (column + 1));

        let mut matches = true;
        for columns_away in 0..max_compare_column_length {
            // Compare against columns across mirror
            for row in 0..pattern.len() {
                if pattern[row][column - columns_away] != pattern[row][column + 1 + columns_away] {
                    matches = false
                }
            }
        }

        if matches { return (column + 1) as i64 }
    }

    -1
}

fn get_horizontal_reflection(pattern: &Vec<Vec<Item>>) -> i64 {
    for row in 0..pattern.len() - 1 {
        // Start comparing
        // Get number of rows we need to compare
        let max_compare_row_length = cmp::min(row + 1, pattern.len() - (row + 1));

        let mut matches = true;
        for rows_away in 0..max_compare_row_length {
            // Compare against columns across mirror
            for column in 0..pattern[row].len() {
                if pattern[row - rows_away][column] != pattern[row + 1 + rows_away][column] {
                    matches = false
                }
            }
        }

        if matches { return (row + 1) as i64 }
    }

    -1
}

