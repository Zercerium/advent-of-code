//

use std::collections::{HashMap, HashSet};

fn adjacent(point: (u32, u32)) -> Vec<(u32, u32)> {
    let mut vec = Vec::new();
    if point.0 > 0 {
        vec.push((point.0 - 1, point.1));
        vec.push((point.0 - 1, point.1 + 1));
    }
    if point.1 > 0 {
        vec.push((point.0, point.1 - 1));
        vec.push((point.0 + 1, point.1 - 1));
    }
    if point.0 > 0 && point.1 > 0 {
        vec.push((point.0 - 1, point.1 - 1));
    }
    vec.push((point.0 + 1, point.1));
    vec.push((point.0, point.1 + 1));
    vec.push((point.0 + 1, point.1 + 1));
    vec
}

fn main() {
    let file = aoc_util::read_input_file(2023, 3);
    let input = file
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|mut line| {
            line.push('.');
            line
        })
        .collect::<Vec<_>>();

    let mut current_number = None;
    let mut with_symbol = false;

    let mut current_adjacent = HashSet::new();
    let mut all_adjacent = HashMap::new();

    let mut res = 0;
    let mut all = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap();
                    if let Some(number) = current_number {
                        current_number = Some(number * 10 + digit);
                    } else {
                        current_number = Some(digit);
                    }
                    let adjacent = adjacent((x as u32, y as u32));
                    for (x, y) in adjacent {
                        let p = input.get(y as usize).and_then(|line| line.get(x as usize));
                        if let Some(p) = p {
                            if p != &'.' && !p.is_ascii_digit() {
                                with_symbol = true;
                                if p == &'*' {
                                    current_adjacent.insert((x, y));
                                }
                            }
                        }
                    }
                }
                _ => {
                    if with_symbol {
                        res += current_number.unwrap();
                        // println!("{}", current_number.unwrap());
                        for p in current_adjacent.iter() {
                            all_adjacent
                                .entry(*p)
                                .or_insert(Vec::new())
                                .push(current_number.unwrap());
                        }
                    }
                    if current_number.is_some() {
                        all += current_number.unwrap();
                        // println!("{}", current_number.unwrap())
                    }
                    current_number = None;
                    with_symbol = false;
                    current_adjacent.clear();
                }
            }
        }
    }

    // Part 1
    println!("{:#?}", res);
    // println!("{:#?}", all);

    // Part 2
    let mut res2 = 0;
    for (_, v) in all_adjacent.iter() {
        if v.len() == 2 {
            res2 += v[0] * v[1];
        }
    }
    println!("{:#?}", res2);
}
