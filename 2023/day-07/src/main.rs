mod helpers;
use crate::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};

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
            FiveKind => 6,
            FourKind => 5,
            FullHouse => 4,
            ThreeKind => 3,
            TwoPair => 2,
            OnePair => 1,
            HighCard => 0
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

        match cards[0] {
            5 => FiveKind,
            4 => FourKind,
            3 => {
                match cards[1] {
                    2 => FullHouse,
                    1 => ThreeKind,
                    _ => panic!("Impossible three-card mapping. The input is wrong!")
                }
            },
            2 => {
                match cards[1] {
                    2 => TwoPair,
                    1 => OnePair,
                    _ => panic!("Impossible two-card mapping. The input is wrong!")
                }
            },
            1 => HighCard,
            _ => panic!("Bad hand type mapping. The input is wrong!")
        }
    }
}

fn main() {
    let input = helpers::get_input(&*helpers::get_path_from_arg());

    let part_1_answer = part_1(&input);
    println!("Part 1 answer: {part_1_answer}.");
}

fn part_1(input: &Vec<String>) -> i64 {
    let mut rounds = get_card_rounds(input);

    // Sort rounds based on their rank
    // If two rounds have the same rank, then use the individual card values
    rounds.sort_by(|a, b | if a.hand_type() == b.hand_type() {
        if a.hands[0] != b.hands[0] {
            b.hands[0].cmp(&a.hands[0])
        } else if a.hands[1] != b.hands[1] {
            b.hands[1].cmp(&a.hands[1])
        } else if a.hands[2] != b.hands[2] {
            b.hands[2].cmp(&a.hands[2])
        } else if a.hands[3] != b.hands[3] {
            b.hands[3].cmp(&a.hands[3])
        } else {
            b.hands[4].cmp(&a.hands[4])
        }
    } else {
        b.hand_type().value().cmp(&a.hand_type().value())
    }
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
