// Day 7: Camel Cards

use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    bid: u16,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = self.hand_type.cmp(&other.hand_type);
        match ord {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(a, b)| a != b)
                .map(|(a, b)| a.cmp(b)),
            _ => Some(ord),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_to_value(c: char, j_is_joker: bool) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' if j_is_joker => 1,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card: {}", c),
    }
}

fn hand_to_hand_type(cards: &Vec<u8>) -> HandType {
    let mut map = HashMap::new();
    for card in cards {
        let count = map.entry(card).or_insert(0 as u8);
        *count += 1;
    }
    match map.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if map.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if map.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Invalid hand: {:?}", cards),
    }
}

fn hand_to_hand_type_with_joker(cards: &Vec<u8>) -> HandType {
    let mut cards = cards.clone();
    cards.retain(|card| *card != 1);
    let joker = 5 - cards.len();
    let mut map = HashMap::new();
    for card in &cards {
        let count = map.entry(card).or_insert(0 as u8);
        *count += 1;
    }
    match map.len() {
        0..=1 => HandType::FiveOfAKind,
        2 => {
            if map.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else if joker == 3 || joker == 2 {
                HandType::FourOfAKind
            } else if joker == 1 && map.values().any(|&v| v == 3) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if map.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else if joker == 2 || joker == 1 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Invalid hand: {:?}", cards),
    }
}

fn main() {
    let file = aoc_util::read_input_file(2023, 7);
    let start = Instant::now();

    let parse_hand = |file: &str, with_joker| {
        let hand_convert = match with_joker {
            true => hand_to_hand_type_with_joker,
            false => hand_to_hand_type,
        };
        file.lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                let bid = bid.parse::<u16>().unwrap();
                let cards = hand
                    .chars()
                    .map(|c| card_to_value(c, with_joker))
                    .collect::<Vec<_>>();
                let hand_type = hand_convert(&cards);
                Hand {
                    cards,
                    bid,
                    hand_type,
                }
            })
            .collect::<Vec<_>>()
    };

    let total_winnings = |hands: &Vec<Hand>| -> usize {
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid as usize * (i + 1))
            .sum()
    };

    let duration = start.elapsed();
    println!("Functions: {:?}", duration);
    // Part 1
    let mut hands = parse_hand(&file, false);
    let duration = start.elapsed();
    println!("Parse 1: {:?}", duration);
    hands.sort_unstable();
    let duration = start.elapsed();
    println!("Sort 1: {:?}", duration);
    let res1 = total_winnings(&hands);
    let duration = start.elapsed();
    println!("Calc 1: {:?}", duration);

    // Part 2
    let mut hands = parse_hand(&file, true);
    hands.sort_unstable();
    let res2 = total_winnings(&hands);

    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joker_5() {
        let cards = vec![1, 1, 1, 1, 1];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FiveOfAKind);
    }

    #[test]
    fn joker_4() {
        let cards = vec![1, 1, 1, 1, 2];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FiveOfAKind);
    }

    #[test]
    fn joker_3() {
        let cards = vec![1, 1, 1, 2, 2];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FiveOfAKind);

        let cards = vec![1, 1, 1, 2, 3];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FourOfAKind);
    }

    #[test]
    fn joker_2() {
        let cards = vec![1, 1, 2, 2, 2];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FiveOfAKind);

        let cards = vec![1, 1, 2, 2, 3];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FourOfAKind);

        let cards = vec![1, 1, 2, 3, 4];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::ThreeOfAKind);
    }

    #[test]
    fn joker_1() {
        let cards = vec![1, 2, 2, 2, 2];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FiveOfAKind);

        let cards = vec![1, 2, 2, 2, 3];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FourOfAKind);

        let cards = vec![1, 2, 2, 3, 3];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::FullHouse);

        let cards = vec![1, 2, 2, 3, 4];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::ThreeOfAKind);

        let cards = vec![1, 2, 3, 4, 5];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::OnePair);
    }

    #[test]
    fn joker_0() {
        let cards = vec![2, 3, 4, 5, 6];
        assert_eq!(hand_to_hand_type_with_joker(&cards), HandType::HighCard);
    }
}
