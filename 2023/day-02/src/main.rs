use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

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
    let mut args: Vec<String> = env::args().collect();

    let mut path: &mut String = &mut String::new();

    if args.len() < 2 {
        print!("Enter path to file: ");
        stdout().flush().expect("Cannot flush buffer");

        stdin().read_line(path).expect("Cannot process input");
        if let Some('\n')=path.chars().next_back() {
            path.pop();
        }
        if let Some('\r')=path.chars().next_back() {
            path.pop();
        }
    } else {
        path = &mut args[1];
    }
    let input: String = fs::read_to_string(path).expect("Couldn't read input file");

    let mut game_id_sum: u32 = 0;
    let mut power_sum = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();

        let game_id: u32 = parts[0][5..].parse().unwrap();

        let mut is_possible = true;
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

                if count > color.minimums() as u32 { is_possible = false }

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
        if is_possible { game_id_sum += game_id }
        power_sum += required_red * required_green * required_blue;
    }

    println!("The sum of the game IDs is {game_id_sum}");
    println!("The sum of the cube powers is {power_sum}")
}
