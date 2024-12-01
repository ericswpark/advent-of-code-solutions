use helpers::*;


fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i32 {
    let mut points_sum: i32 = 0;
    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let winning_numbers: Vec<i32> = parse_input_to_scratchcard_numbers(numbers[0]);
        let elf_numbers: Vec<i32> = parse_input_to_scratchcard_numbers(numbers[1]);

        // Get number of matches
        let match_count = get_match_count(winning_numbers, elf_numbers);

        points_sum += if match_count >= 1 { 2_i32.checked_pow((match_count - 1) as u32).unwrap() } else { 0 };
    }
    points_sum
}

fn part_2(input: &Vec<String>) -> i32 {
    let mut cards_count_by_id: Vec<i32> = vec![0; get_card_count(input) as usize];

    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        let card_id = parts[0][5..].trim().parse::<i32>().unwrap();
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let winning_numbers: Vec<i32> = parse_input_to_scratchcard_numbers(numbers[0]);
        let elf_numbers: Vec<i32> = parse_input_to_scratchcard_numbers(numbers[1]);

        cards_count_by_id[card_id as usize - 1] += 1;

        // Get number of matches
        let match_count = get_match_count(winning_numbers, elf_numbers);

        // Increment number of matches, repeat for number of matches in current ID
        for _ in 0..cards_count_by_id[card_id as usize - 1] {
            for i in 0..match_count {
                cards_count_by_id[card_id as usize + i] += 1;
            }
        }
    }

    cards_count_by_id.iter().sum()
}

fn get_match_count(winning_numbers: Vec<i32>, elf_numbers: Vec<i32>) -> usize {
    let mut match_count = 0;
    for i in 0..elf_numbers.len() {
        for j in 0..winning_numbers.len() {
            if elf_numbers.get(i) == winning_numbers.get(j) {
                match_count += 1;
                break;
            }
        }
    }
    match_count
}

fn get_card_count(input: &Vec<String>) -> i32 {
    let mut last_card_id = 0;
    for line in input {
        let card_id = line.split(": ").collect::<Vec<_>>()[0][5..].trim().parse::<i32>().unwrap();
        last_card_id = card_id;
    }

    last_card_id
}

fn parse_input_to_scratchcard_numbers(input: &str) -> Vec<i32> {
    input.split(' ').filter(|&x| !x.is_empty()).map(|s: &str| s.parse::<i32>().unwrap() ).collect()
}