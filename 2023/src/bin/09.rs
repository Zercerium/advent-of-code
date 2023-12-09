// Day 9: Mirage Maintenance

use std::{collections::VecDeque, time::Instant};

fn main() {
    let file = aoc_util::read_input_file(2023, 9);
    let start = Instant::now();

    let histories = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum1 = histories
        .iter()
        .map(|history| extrapolate(&history).0)
        .sum::<i64>();

    let sum2 = histories
        .iter()
        .map(|history| extrapolate(&history).1)
        .sum::<i64>();

    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}

fn extrapolate(history: &[i64]) -> (i64, i64) {
    let mut extrapolate = Vec::new();
    extrapolate.push(VecDeque::from(history.to_vec()));
    loop {
        let mut last = extrapolate.iter().last().unwrap().clone();
        for i in 0..last.len() - 1 {
            last[i] = last[i + 1] - last[i];
        }
        last.pop_back();
        extrapolate.push(last);
        if extrapolate.iter().last().unwrap().iter().all(|&n| n == 0) {
            break;
        }
    }
    extrapolate.iter_mut().last().unwrap().push_back(0);
    for i in (0..extrapolate.len() - 1).rev() {
        let last = *extrapolate.get(i + 1).unwrap().iter().last().unwrap();
        let vec = extrapolate.get_mut(i).unwrap();
        let new = vec.iter().last().unwrap() + last;
        vec.push_back(new);
    }

    extrapolate.iter_mut().last().unwrap().push_front(0);
    for i in (0..extrapolate.len() - 1).rev() {
        let first = *extrapolate.get(i + 1).unwrap().iter().next().unwrap();
        let vec = extrapolate.get_mut(i).unwrap();
        let new = vec.iter().next().unwrap() - first;
        vec.push_front(new);
    }

    // dbg!(&extrapolate);

    (
        *extrapolate.first().unwrap().iter().last().unwrap(),
        *extrapolate.first().unwrap().iter().next().unwrap(),
    )
}
