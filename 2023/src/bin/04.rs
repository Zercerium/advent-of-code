// Day 4: Scratchcards

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u8>,
    my_numbers: Vec<u8>,
}

fn parse(s: &str) -> Vec<Card> {
    s.lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning, mine) = numbers.split_once(" | ").unwrap();
            // println!("{:#?} {:#?}", winning, mine);
            let mut winning_numbers: Vec<_> = winning
                .split_whitespace()
                // .inspect(|n| println!("{}", n))
                .map(|n| n.parse().unwrap())
                .collect();
            winning_numbers.sort_unstable();
            let mut my_numbers: Vec<_> = mine
                .split_whitespace()
                // .inspect(|n| println!("{}", n))
                .map(|n| n.trim().parse().unwrap())
                .collect();
            my_numbers.sort_unstable();
            Card {
                winning_numbers,
                my_numbers,
            }
        })
        .collect()
}

fn main() {
    let file = aoc_util::read_input_file(2023, 4);
    let cards = parse(&file);
    let res1 = cards
        .iter()
        .map(|card| {
            let mut numbers = Vec::from(card.winning_numbers.clone());
            numbers.extend(card.my_numbers.clone());
            numbers.sort_unstable();
            numbers
        })
        .map(|mut numbers| {
            let len = numbers.len();
            numbers.dedup();
            let len2 = numbers.len();
            let double = len - len2;
            if double == 0 {
                return 0;
            }
            (2 as u32).pow(double as u32 - 1)
        })
        .sum::<u32>();
    println!("{:#?}", res1);

    // Part 2
    let matches = cards
        .iter()
        .map(|card| {
            let mut numbers = Vec::from(card.winning_numbers.clone());
            numbers.extend(card.my_numbers.clone());
            numbers.sort_unstable();
            numbers
        })
        .map(|mut numbers| {
            let len = numbers.len();
            numbers.dedup();
            let len2 = numbers.len();
            let double = len - len2;
            double
        })
        .collect::<Vec<_>>();

    let mut card_counts = vec![1; matches.len()];
    for card_id in 0..card_counts.len() {
        let card_count = card_counts[card_id];
        for n in 1..=matches[card_id] {
            if let Some(c) = card_counts.get_mut(card_id + n) {
                *c += 1 * card_count;
            }
        }
    }
    let res2 = card_counts.iter().sum::<usize>();
    // println!("{:#?}", card_counts);
    println!("{:#?}", res2);
}
