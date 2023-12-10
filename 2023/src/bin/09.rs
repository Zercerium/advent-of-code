// Day 9: Mirage Maintenance

use std::time::Instant;

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
    extrapolate.push(history.to_vec());
    loop {
        let mut last = extrapolate.iter().last().unwrap().clone();
        for i in 0..last.len() - 1 {
            last[i] = last[i + 1] - last[i];
        }
        last.pop();
        extrapolate.push(last);
        if extrapolate.iter().last().unwrap().iter().all(|&n| n == 0) {
            break;
        }
    }

    let part1;
    let part2;

    part1 = extrapolate
        .iter()
        .rev()
        .map(|row| *row.iter().last().unwrap())
        // .inspect(|n| println!("{}", n))
        .reduce(|acc, b| b + acc)
        .unwrap();

    part2 = extrapolate
        .iter()
        .rev()
        .map(|row| *row.iter().next().unwrap())
        .reduce(|acc, b| b - acc)
        .unwrap();

    (part1, part2)
}
