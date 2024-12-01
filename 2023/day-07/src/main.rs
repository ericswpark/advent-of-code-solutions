mod tests;

use helpers::*;


#[derive(PartialEq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn value(&self) -> i16 {
        match *self {
            HandType::FiveKind => 6,
            HandType::FourKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0
        }
    }
}

struct Round {
    hands: Vec<char>,
    bid: i32
}

impl Round {
    fn hand_type(&self) -> HandType {
        let mut cards: Vec<i16> = vec![0; 13];
        for hand in &self.hands {
            cards[get_card_value_mapping(*hand) as usize] += 1;
        }

        cards.sort();
        cards.reverse();

        card_count_to_hand_type(cards[0], cards[1])
    }

    fn hand_type_with_joker(&self) -> HandType {
        let mut cards: Vec<i16> = vec![0; 13];
        for hand in &self.hands {
            cards[get_card_value_mapping_with_joker(*hand) as usize] += 1;
        }

        // Back up joker count for now
        let joker_count = cards[0];
        cards[0] = 0;

        cards.sort();
        cards.reverse();

        // Add joker count to the highest card number
        cards[0] += joker_count;

        card_count_to_hand_type(cards[0], cards[1])
    }
}

fn card_count_to_hand_type(first: i16, second: i16) -> HandType {
    match first {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            match second {
                2 => HandType::FullHouse,
                1 => HandType::ThreeKind,
                _ => panic!("Impossible three-card mapping. The input is wrong!")
            }
        },
        2 => {
            match second {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => panic!("Impossible two-card mapping. The input is wrong!")
            }
        },
        1 => HandType::HighCard,
        _ => panic!("Bad hand type mapping. The input is wrong!")
    }
}

fn main() {
    let input = get_input(&get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}");

    let part_2_answer = part_2(&input);
    println!("Part 2 answer: {part_2_answer}");
}

fn part_1(input: &Vec<String>) -> i64 {
    let mut rounds = get_card_rounds(input);

    // Sort rounds based on their rank
    // If two rounds have the same rank, then use the individual card values
    for i in 0..=4 {
        rounds.sort_by(|a, b| get_card_value_mapping(a.hands[4-i]).cmp(&get_card_value_mapping(b.hands[4 - i])))
    }
    rounds.sort_by(|a, b | a.hand_type().value().cmp(&b.hand_type().value()));

    let mut total_winnings: i64 = 0;

    for (round_index, round) in rounds.iter().enumerate() {
        total_winnings += ((round_index + 1) as i32 * round.bid) as i64;
    }

    total_winnings
}

fn part_2(input: &Vec<String>) -> i64 {
    let mut rounds = get_card_rounds(input);

    // Sort rounds based on their rank
    // If two rounds have the same rank, then use the individual card values
    for i in 0..=4 {
        rounds.sort_by(|a, b|
            get_card_value_mapping_with_joker(a.hands[4-i])
                .cmp(&get_card_value_mapping_with_joker(b.hands[4 - i]))
        )
    }
    rounds.sort_by(|a, b |
        a.hand_type_with_joker().value()
            .cmp(&b.hand_type_with_joker().value())
    );

    let mut total_winnings: i64 = 0;

    for (round_index, round) in rounds.iter().enumerate() {
        total_winnings += ((round_index + 1) as i32 * round.bid) as i64;
    }

    total_winnings
}

fn get_card_rounds(input: &Vec<String>) -> Vec<Round> {
    let mut ret = Vec::new();

    for line in input {
        let parts: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();

        let mut cards: Vec<char> = Vec::new();
        for c in parts[0].chars() {
            cards.push(c);
        }

        ret.push(Round {
            hands: cards,
            bid: parts[1].parse::<i32>().unwrap()
        })

    }

    ret
}

fn get_card_value_mapping(c: char) -> i16 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => { panic!("Bad card mapping!") }
    }
}

fn get_card_value_mapping_with_joker(c: char) -> i16 {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => { panic!("Bad card mapping!") }
    }
}
