use helpers::*;

mod tests;

aoc_main!();

fn part_1(input: &[String]) -> i64 {
    let mut current = 50;
    let mut password = 0;

    for line in input {
        let direction = line.chars().next().unwrap();
        let distance = line[1..].parse::<i64>().unwrap();
        match direction {
            'L' => current -= distance,
            'R' => current += distance,
            _ => unreachable!("Shouldn't happen, direction is either L or R"),
        }

        // Loop around
        while current > 99 {
            current -= 100;
        }
        while current < 0 {
            current += 100;
        }

        if current == 0 {
            password += 1;
        }
    }

    password
}

fn part_2(input: &[String]) -> i64 {
    let mut current = 50;
    let mut password = 0;

    for line in input {
        let direction = line.chars().next().unwrap();
        let mut distance = line[1..].parse::<i64>().unwrap();
        match direction {
            'L' => {
                while distance > 0 {
                    current -= 1;
                    if current == -1 {
                        current = 99
                    };
                    if current == 0 {
                        password += 1;
                    }
                    distance -= 1;
                }
            }
            'R' => {
                while distance > 0 {
                    current += 1;
                    if current == 100 {
                        current = 0
                    };
                    if current == 0 {
                        password += 1;
                    }
                    distance -= 1;
                }
            }
            _ => unreachable!("Shouldn't happen, direction is either L or R"),
        }
    }

    password
}
