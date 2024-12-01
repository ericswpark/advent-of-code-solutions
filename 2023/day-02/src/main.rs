mod tests;

use helpers::*;

enum Color {
    RED,
    GREEN,
    BLUE
}

impl Color {
    fn minimums(&self) -> i32 {
        match *self {
            Color::RED => 12,
            Color::GREEN => 13,
            Color::BLUE => 14,
        }
    }
}

fn get_color_mapping(s: &str) -> Color {
    match s {
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        _ => panic!("Invalid color mapping")
    }
}


fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &[String]) -> u32 {
    let mut game_id_sum: u32 = 0;

    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        let game_id: u32 = parts[0][5..].parse().unwrap();
        let mut is_possible = true;
        let reveal_sets: Vec<_> = parts[1].split("; ").collect();

        for reveal_set in reveal_sets {
            let blocks: Vec<_> = reveal_set.split(", ").collect();
            for block in blocks {
                let data: Vec<_> = block.split(' ').collect();
                let count: u32 = data[0].parse().unwrap();
                let color: Color = get_color_mapping(data[1]);

                if count > color.minimums() as u32 { is_possible = false }
            }
        }
        if is_possible { game_id_sum += game_id }
    }
    game_id_sum
}

fn part_2(input: &[String]) -> u32 {
    let mut power_sum = 0;

    for line in input {
        let parts: Vec<_> = line.split(": ").collect();

        let mut required_red = 0;
        let mut required_green = 0;
        let mut required_blue = 0;

        let reveal_sets: Vec<_> = parts[1].split("; ").collect();

        for reveal_set in reveal_sets {
            let blocks: Vec<_> = reveal_set.split(", ").collect();
            for block in blocks {
                let data: Vec<_> = block.split(' ').collect();
                let count: u32 = data[0].parse().unwrap();
                let color: Color = get_color_mapping(data[1]);

                match color {
                    Color::RED => {
                        if count > required_red { required_red = count }
                    }
                    Color::GREEN => {
                        if count > required_green { required_green = count }
                    }
                    Color::BLUE => {
                        if count > required_blue { required_blue = count }
                    }
                }
            }


        }
        power_sum += required_red * required_green * required_blue;
    }
    power_sum
}