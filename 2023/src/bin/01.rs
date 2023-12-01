use std::time::Instant;

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

// Day 1: Trebuchet?!
fn main() {
    let file = aoc_util::read_input_file(2023, 1);

    // Part 1
    let start = Instant::now();
    let res: u32 = file
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap();
            let first = first.to_digit(10).unwrap();
            let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
            let last = last.to_digit(10).unwrap();
            first * 10 + last
        })
        .sum();
    let duration = start.elapsed();
    println!("Time elapsed in Part 1 is: {:?}", duration);

    println!("Result Part 1: {:?}", res);

    // Part 2
    let start = Instant::now();
    let res: u32 = file
        .lines()
        .map(|line| {
            let line_bytes = line.as_bytes();

            let first = line_bytes
                .iter()
                .enumerate()
                .find_map(|(index, _)| find_first_number(&line_bytes[index..]));

            let last = line_bytes
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, _)| find_last_number(&line_bytes[..=index]));

            first.unwrap() * 10 + last.unwrap()
        })
        .sum();
    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Result Part 2: {:?}", res)
}

fn find_first_number(s: &[u8]) -> Option<u32> {
    let first = s.first().unwrap() - b'0';
    if first < 10 {
        return Some(first as u32);
    }

    NUMBERS
        .into_iter()
        .enumerate()
        .find_map(|(index, number)| s.starts_with(number).then(|| index as u32 + 1))
}

fn find_last_number(s: &[u8]) -> Option<u32> {
    let first = s.last().unwrap() - b'0';
    if first < 10 {
        return Some(first as u32);
    }

    NUMBERS
        .into_iter()
        .enumerate()
        .find_map(|(index, number)| s.ends_with(number).then(|| index as u32 + 1))
}
