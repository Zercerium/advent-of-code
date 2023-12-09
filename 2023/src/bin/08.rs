// Day 8: Haunted Wasteland

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc_util::num::Integer;

#[derive(Debug)]
struct Node {
    left: [u8; 3],
    right: [u8; 3],
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let file = aoc_util::read_input_file(2023, 8);
    let start = Instant::now();

    let (directions, network) = file.split_once("\n\n").unwrap();
    let directions: Vec<_> = directions
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();
    let network: HashMap<[u8; 3], _> = network
        .lines()
        .map(|line| {
            let (key, paths) = line.split_once(" = ").unwrap();
            let node = Node {
                left: paths.as_bytes()[1..4].try_into().unwrap(),
                right: paths.as_bytes()[6..9].try_into().unwrap(),
            };
            (key.as_bytes().try_into().unwrap(), node)
        })
        .collect();

    // dbg!(&directions);
    // dbg!(&network);

    let mut current_pos = b"AAA";
    let mut current_dir_index = 0;
    let mut count = 0;
    while current_pos != b"ZZZ" {
        let dir = &directions[current_dir_index];
        current_pos = match dir {
            Direction::Left => &network.get(current_pos).unwrap().left,
            Direction::Right => &network.get(current_pos).unwrap().right,
        };
        current_dir_index = (current_dir_index + 1) % directions.len();
        count += 1;
    }

    let res1 = count;

    let currents_poss = network
        .keys()
        .filter(|key| key[2] == b'A')
        .collect::<Vec<_>>();

    let z: Vec<_> = currents_poss
        .iter()
        .map(|&pos| find_repetition(pos, &network, &directions))
        .collect();

    let count = z.iter().map(|z| z.0).reduce(|a, b| a.lcm(&b)).unwrap();
    let res2 = count;

    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn find_repetition(
    start: &[u8; 3],
    network: &HashMap<[u8; 3], Node>,
    directions: &[Direction],
) -> (usize, Vec<usize>) {
    let mut visited = HashSet::new();

    let mut with_z = Vec::new();

    let mut current_pos = start;
    let mut current_dir_index = 0;
    loop {
        let known = !visited.insert((current_pos, current_dir_index));
        if known {
            break;
        }
        if current_pos[2] == b'Z' {
            with_z.push(visited.len() - 1);
        }
        current_pos = match &directions[current_dir_index] {
            Direction::Left => &network.get(current_pos).unwrap().left,
            Direction::Right => &network.get(current_pos).unwrap().right,
        };
        current_dir_index = (current_dir_index + 1) % directions.len();
    }
    let v = visited.get(&(current_pos, current_dir_index)).unwrap();
    (visited.len() - v.1, with_z)
}
