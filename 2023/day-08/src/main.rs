mod helpers;
mod tests;


struct MapNode {
    name: String,
    left: String,
    right: String,
    left_index: Option<i32>,
    right_index: Option<i32>,
}

impl MapNode {
    fn new(name: String, left: String, right: String) -> Self {
        MapNode {
            name, left, right, left_index: None, right_index: None
        }
    }
}

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let sequence = get_sequence(&input[0]);

    let mut map: Vec<MapNode> = get_map(&input[2..]);

    let mut current_node_index = map.iter().position(|r| r.name == "AAA").unwrap();
    let mut current_sequence_index = 0;
    let mut count = 0;

    loop {
        // If current node is ZZZ, break
        if map[current_node_index].name == "ZZZ" { break }

        // Go to the next node based on the current sequence
        let next_direction = sequence[current_sequence_index];
        current_sequence_index = if current_sequence_index + 1 < sequence.len() { current_sequence_index + 1 } else { 0 };

        match next_direction {
            'L' => {
                let next_node_name = &map[current_node_index].left;

                if map[current_node_index].left_index == None {
                    map[current_node_index].left_index = Some(map.iter().position(|r| r.name == *next_node_name).unwrap() as i32);
                }

                current_node_index = map[current_node_index].left_index.unwrap() as usize;

            },
            'R' => {
                let next_node_name = &map[current_node_index].right;

                if map[current_node_index].right_index == None {
                    map[current_node_index].right_index = Some(map.iter().position(|r| r.name == *next_node_name).unwrap() as i32);
                }

                current_node_index = map[current_node_index].right_index.unwrap() as usize;
            },
            _ => panic!("Bad sequence!"),
        }

        count += 1;
    }

    count
}

fn part_2(_input: &Vec<String>) -> i64 {
    0
}

fn get_sequence(input: &String) -> Vec<char> {
    input.chars().collect()
}

fn get_map(input: &[String]) -> Vec<MapNode> {
    let mut map = Vec::new();

    for line in input {
        let [start_name, next_direction]: [&str; 2] = line.split(" = ").collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let left_name = &next_direction[1..=3];
        let right_name = &next_direction[6..=8];

        map.push(MapNode::new(
            start_name.parse().unwrap(),
            left_name.parse().unwrap(),
            right_name.parse().unwrap()
        ));
    }

    map
}

