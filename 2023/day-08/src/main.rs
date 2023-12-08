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

        traverse_next_node(&mut map, &mut current_node_index, next_direction);

        count += 1;
    }

    count
}

fn part_2(input: &Vec<String>) -> i64 {
    let sequence = get_sequence(&input[0]);

    let mut map: Vec<MapNode> = get_map(&input[2..]);

    let current_node_indices = map.iter().enumerate().filter(|(_, r)| r.name.ends_with('A')).map(|(index, _)| index).collect::<Vec<_>>();
    let mut steps_for_index: Vec<i64> = vec![0; current_node_indices.len()];

    // For each node, figure out how many steps it would take to get to the Z part
    for (index, &node_index) in current_node_indices.iter().enumerate() {
        let mut node_index = node_index;
        let mut current_sequence_index = 0;
        let mut count = 0;

        loop {
            if map[node_index].name.ends_with('Z') { break }

            // Go to the next node based on the current sequence
            let next_direction = sequence[current_sequence_index];
            current_sequence_index = if current_sequence_index + 1 < sequence.len() { current_sequence_index + 1 } else { 0 };

            traverse_next_node(&mut map, &mut node_index, next_direction);

            count += 1;
        }
        steps_for_index[index] = count;
    }

    lcm_vec(steps_for_index)
}

fn traverse_next_node(map: &mut Vec<MapNode>, node_index: &mut usize, next_direction: char) {
    match next_direction {
        'L' => {
            let next_node_name = &map[*node_index].left;

            if map[*node_index].left_index == None {
                map[*node_index].left_index = Some(map.iter().position(|r| r.name == *next_node_name).unwrap() as i32);
            }

            *node_index = map[*node_index].left_index.unwrap() as usize;
        },
        'R' => {
            let next_node_name = &map[*node_index].right;

            if map[*node_index].right_index == None {
                map[*node_index].right_index = Some(map.iter().position(|r| r.name == *next_node_name).unwrap() as i32);
            }

            *node_index = map[*node_index].right_index.unwrap() as usize;
        },
        _ => panic!("Bad sequence!"),
    }
}

fn lcm_vec(input: Vec<i64>) -> i64 {
    let mut total = input[0];

    for i in 0..input.len() {
        total = lcm(total, input[i]);
    }

    total
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut max = if a > b { a } else { b };
    let mut min = if a < b { a } else { b };

    loop {
        let remainder = max % min;
        if remainder == 0 {
            return min
        }

        max = min;
        min = remainder;
    }
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

