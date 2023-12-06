// Day 5: If You Give A Seed A Fertilizer

fn main() {
    let file = aoc_util::read_input_file(2023, 6);
    let input = file
        .lines()
        .map(|line| {
            let (_, ints) = line.split_once(": ").unwrap();
            ints.split_whitespace()
                .map(|int| int.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    let input = input[0].iter().zip(input[1].iter()).collect::<Vec<_>>();
    let res: usize = input
        .iter()
        .map(|race| {
            (0..=*race.0)
                .map(|hold| (race.0 - hold) * hold)
                .filter(|distance| distance > race.1)
                .count()
        })
        .product();
    println!("Part 1: {}", res);

    let input = file
        .lines()
        .map(|line| {
            let (_, ints) = line.split_once(": ").unwrap();
            ints.replace(" ", "").parse().unwrap()
        })
        .collect::<Vec<usize>>();
    dbg!(&input);
    let distance = input[1] as f64;
    let duration = input[0] as f64;
    let x1 = (1. / 2.) * (duration - f64::sqrt(duration.powi(2) - 4. * distance));
    let x2 = (1. / 2.) * (duration + f64::sqrt(duration.powi(2) - 4. * distance));
    let x1 = x1.ceil() as usize;
    let x2 = x2.floor() as usize;
    println!("Part 2: {}", x2 - x1 + 1);
}
