mod helpers;


struct MapNode {
    name: String,
    left: String,
    right: String
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

    let map: Vec<MapNode> = get_map(&input[2..]);

    let mut current_node_index = 0;
    let mut current_sequence_index = 0;
    let mut count = 0;

    loop {
        // Get current node
        let current = &map[current_node_index];

        // If current node is ZZZ, break
        if current.name == "ZZZ" { break }

        // Go to the next node based on the current sequence
        let next_direction = sequence[current_sequence_index];
        current_sequence_index = if current_sequence_index + 1 < sequence.len() { current_sequence_index + 1 } else { 0 };

        match next_direction {
            'L' => {
                let next_node_name = &current.left;
                current_node_index = map.iter().position(|r| r.name == *next_node_name).unwrap();
            },
            'R' => {
                let next_node_name = &current.right;
                current_node_index = map.iter().position(|r| r.name == *next_node_name).unwrap();
            },
            _ => panic!("Bad sequence!"),
        }

        count += 1
    }

    count
}

fn part_2(input: &Vec<String>) -> i64 {
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

        map.push(MapNode{
            name: start_name.parse().unwrap(),
            left: left_name.parse().unwrap(),
            right: right_name.parse().unwrap()
        });
    }

    map
}

