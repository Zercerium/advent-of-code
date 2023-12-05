// Day 5: If You Give A Seed A Fertilizer

use std::ops::Range;

#[derive(Debug)]
struct Map {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Debug)]
struct BMap {
    source_range: Range<u64>,
    offset: i64,
}

fn main() {
    let file = aoc_util::read_input_file(2023, 5);
    let (seeds, maps) = parse(&file);

    let min = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, map| {
                // println!("{}", acc);
                map.iter()
                    .find(|m| (m.source..m.source + m.length).contains(&acc))
                    .map_or_else(
                        || acc,
                        |m| (acc as i64 + m.destination as i64 - m.source as i64) as u64,
                    )
            })
        })
        // .inspect(|i| println!("{}", i))
        .min();
    println!("Part 1: {:#?}", min.unwrap());

    // Part 2
    let seeds = seeds
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();
    // dbg!(&seeds);
    let maps = maps
        .iter()
        .map(|map| {
            map.iter()
                .map(|m| BMap {
                    source_range: m.source..(m.source + m.length),
                    offset: m.destination as i64 - m.source as i64,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // dbg!(&maps);
    let min = seeds
        .iter()
        .flat_map(|seed| {
            maps.iter().fold(vec![seed.clone()], |mut acc, map| {
                // dbg!(&acc);
                // dbg!(&map);
                let mut new_acc = Vec::new();
                for range in &mut acc {
                    // dbg!(&range);
                    let mut last_range = range.clone();
                    while range.end - range.start > 1 {
                        // dbg!(&range);
                        // dbg!(&new_acc);
                        let fitting_range = map
                            .iter()
                            .enumerate()
                            .find(|&r| r.1.source_range.contains(&range.start));

                        let new_range = if let Some(fitting_range) = fitting_range {
                            let current_map_end = map[fitting_range.0].source_range.end;
                            let end = if current_map_end > range.end {
                                range.start = range.end;
                                range.end
                            } else {
                                range.start = current_map_end;
                                current_map_end
                            };
                            // dbg!(end);
                            let new = (last_range.start as i64 + map[fitting_range.0].offset) as u64
                                ..(end as i64 + map[fitting_range.0].offset) as u64;
                            new
                        } else {
                            let end = map
                                .iter()
                                .enumerate()
                                .rev()
                                .find(|&r| range.start >= r.1.source_range.end);
                            let end = if let Some(end) = end {
                                let next_map_start = map.get(end.0 + 1);
                                if let Some(m) = next_map_start {
                                    let next_map_start = m.source_range.start;
                                    if next_map_start > range.end {
                                        // dbg!(&range.end);
                                        range.end
                                    } else {
                                        // dbg!(&next_map_start);
                                        next_map_start
                                    }
                                } else {
                                    // dbg!(&range.end);
                                    range.end
                                }
                            } else {
                                range.end
                            };
                            // dbg!(end);
                            let new = range.start..end;
                            range.start = end;
                            new
                        };
                        new_acc.push(new_range);
                        if last_range.start >= range.start {
                            panic!("infinite loop")
                        }
                        last_range = range.clone();
                        // dbg!(&range);
                    }
                }
                new_acc
            })
        })
        .map(|range| range.start)
        // .inspect(|i| println!("{}", i))
        .min();
    println!("Part 2: {:#?}", min.unwrap());
}

fn parse(file: &str) -> (Vec<u64>, Vec<Vec<Map>>) {
    let paragraphs = file.split("\n\n").collect::<Vec<_>>();
    let mut paragraphs = paragraphs.iter();

    let seeds = paragraphs.next().unwrap().split_once(": ").unwrap().1;
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let maps = paragraphs
        .map(|&p| {
            let mut entry = p
                .lines()
                .skip(1)
                .map(|line| {
                    let entry = line.split_whitespace().collect::<Vec<_>>();
                    Map {
                        source: entry[1].parse().unwrap(),
                        destination: entry[0].parse().unwrap(),
                        length: entry[2].parse().unwrap(),
                    }
                })
                .collect::<Vec<_>>();
            entry.sort_unstable_by_key(|m| m.source);
            entry
        })
        .collect::<Vec<_>>();
    (seeds, maps)
}
