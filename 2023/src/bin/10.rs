// Day 9: Mirage Maintenance

use std::{collections::HashSet, process::exit, time::Instant};

use aoc_util::map::Map2D;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    Ground,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            '.' | 'I' => Pipe::Ground,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            'F' => Pipe::SouthEast,
            '7' => Pipe::SouthWest,
            'S' => Pipe::Start,
            _ => panic!("Invalid pipe character: {}", c),
        }
    }

    fn opposite_pos(&self, pos: Position) -> Position {
        match self {
            Pipe::Vertical => match pos {
                Position::North => Position::South,
                Position::South => Position::North,
                _ => panic!("Invalid position for vertical pipe: {:?}", pos),
            },
            Pipe::Horizontal => match pos {
                Position::East => Position::West,
                Position::West => Position::East,
                _ => panic!("Invalid position for horizontal pipe: {:?}", pos),
            },
            Pipe::Ground => panic!("Invalid pipe: {:?}", self),
            Pipe::NorthEast => match pos {
                Position::North => Position::East,
                Position::East => Position::North,
                _ => panic!("Invalid position for north-east pipe: {:?}", pos),
            },
            Pipe::NorthWest => match pos {
                Position::North => Position::West,
                Position::West => Position::North,
                _ => panic!("Invalid position for north-west pipe: {:?}", pos),
            },
            Pipe::SouthEast => match pos {
                Position::South => Position::East,
                Position::East => Position::South,
                _ => panic!("Invalid position for south-east pipe: {:?}", pos),
            },
            Pipe::SouthWest => match pos {
                Position::South => Position::West,
                Position::West => Position::South,
                _ => panic!("Invalid position for south-west pipe: {:?}", pos),
            },
            Pipe::Start => panic!("Invalid pipe: {:?}", self),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    North,
    South,
    East,
    West,
}

impl Position {
    fn opposite(&self) -> Self {
        match self {
            Position::North => Position::South,
            Position::South => Position::North,
            Position::East => Position::West,
            Position::West => Position::East,
        }
    }
}

fn next_pipe(pipe: Pipe, position: Position) -> Position {
    match pipe {
        Pipe::Vertical => match position {
            Position::North => Position::South,
            Position::South => Position::North,
            _ => panic!("Invalid position for vertical pipe: {:?}", position),
        },
        Pipe::Horizontal => match position {
            Position::East => Position::West,
            Position::West => Position::East,
            _ => panic!("Invalid position for horizontal pipe: {:?}", position),
        },
        Pipe::Ground => panic!("Invalid pipe: {:?}", pipe),
        Pipe::NorthEast => match position {
            Position::North => Position::East,
            Position::East => Position::North,
            _ => panic!("Invalid position for north-east pipe: {:?}", position),
        },
        Pipe::NorthWest => match position {
            Position::North => Position::West,
            Position::West => Position::North,
            _ => panic!("Invalid position for north-west pipe: {:?}", position),
        },
        Pipe::SouthEast => match position {
            Position::South => Position::East,
            Position::East => Position::South,
            _ => panic!("Invalid position for south-east pipe: {:?}", position),
        },
        Pipe::SouthWest => match position {
            Position::South => Position::West,
            Position::West => Position::South,
            _ => panic!("Invalid position for south-west pipe: {:?}", position),
        },
        Pipe::Start => panic!("Invalid pipe: {:?}", pipe),
    }
}

fn is_connected(pipe: Pipe, position: Position) -> bool {
    match pipe {
        Pipe::Vertical => match position {
            Position::North => true,
            Position::South => true,
            _ => false,
        },
        Pipe::Horizontal => match position {
            Position::East => true,
            Position::West => true,
            _ => false,
        },
        Pipe::Ground => false,
        Pipe::NorthEast => match position {
            Position::North => true,
            Position::East => true,
            _ => false,
        },
        Pipe::NorthWest => match position {
            Position::North => true,
            Position::West => true,
            _ => false,
        },
        Pipe::SouthEast => match position {
            Position::South => true,
            Position::East => true,
            _ => false,
        },
        Pipe::SouthWest => match position {
            Position::South => true,
            Position::West => true,
            _ => false,
        },
        Pipe::Start => false,
    }
}

fn discover_loop(pipes: &mut Map2D<Pipe>) -> (usize, usize) {
    let start_pos = pipes.find_item(Pipe::Start).unwrap();

    let north = start_pos.north_checked();
    let south = start_pos.south();
    let east = start_pos.east();
    let west = start_pos.west_checked();

    let mut adjacent_pipes = vec![(east, Position::West), (south, Position::North)];
    if let Some(north) = north {
        adjacent_pipes.push((north, Position::South));
    }
    if let Some(west) = west {
        adjacent_pipes.push((west, Position::East));
    }

    let sum: Vec<_> = adjacent_pipes
        .iter()
        .filter(|(p, o)| pipes.get2(*p).map_or(false, |p| is_connected(*p, *o)))
        .collect();

    assert_eq!(sum.len(), 2);
    // let mut current_pos = (sum[0].0, sum[0].1);
    let mut current_pos = (start_pos, sum[0].1.opposite());

    let mut left_turns = 0;
    let mut right_turns = 0;

    let start_pos1 = sum[0].1.opposite();
    let start_pos2 = sum[1].1.opposite();

    let start_pipe = match (start_pos1, start_pos2) {
        (Position::North, Position::South) => Pipe::Vertical,
        (Position::North, Position::East) => Pipe::NorthEast,
        (Position::North, Position::West) => Pipe::NorthWest,
        (Position::South, Position::North) => Pipe::Vertical,
        (Position::South, Position::East) => Pipe::SouthEast,
        (Position::South, Position::West) => Pipe::SouthWest,
        (Position::East, Position::North) => Pipe::NorthEast,
        (Position::East, Position::South) => Pipe::SouthEast,
        (Position::East, Position::West) => Pipe::Horizontal,
        (Position::West, Position::North) => Pipe::NorthWest,
        (Position::West, Position::South) => Pipe::SouthWest,
        (Position::West, Position::East) => Pipe::Horizontal,
        _ => panic!("Invalid start position: {:?}", (start_pos1, start_pos2)),
    };

    // dbg!(start_pipe);

    pipes.set(start_pos, start_pipe);

    let mut pipes_in_loop = HashSet::new();

    loop {
        pipes_in_loop.insert(current_pos.0);
        let pipe = pipes.get2(current_pos.0).unwrap();

        match *pipe {
            Pipe::NorthEast => match current_pos.1 {
                Position::North => left_turns += 1,
                Position::East => right_turns += 1,
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::NorthWest => match current_pos.1 {
                Position::North => right_turns += 1,
                Position::West => left_turns += 1,
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::SouthEast => match current_pos.1 {
                Position::East => left_turns += 1,
                Position::South => right_turns += 1,
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::SouthWest => match current_pos.1 {
                Position::South => left_turns += 1,
                Position::West => right_turns += 1,
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::Vertical | Pipe::Horizontal => (),
            _ => panic!("Invalid pipe: {:?}", pipe),
        }

        let next_pos = next_pipe(*pipe, current_pos.1);
        let coord = match next_pos {
            Position::North => current_pos.0.north(),
            Position::South => current_pos.0.south(),
            Position::East => current_pos.0.east(),
            Position::West => current_pos.0.west(),
        };
        let entry_pos = next_pos.opposite();
        current_pos = (coord, entry_pos);
        if current_pos.0 == start_pos {
            break;
        }
    }

    // dbg!(left_turns, right_turns);

    let turns: i32 = right_turns - left_turns;
    assert_eq!(turns.abs(), 4);
    if turns != 4 {
        let pipe = pipes.get2(current_pos.0).unwrap();
        current_pos.1 = pipe.opposite_pos(current_pos.1);
    }

    let mut queue = Vec::new();
    let mut inside = HashSet::new();

    loop {
        let pipe = pipes.get2(current_pos.0).unwrap();

        match *pipe {
            Pipe::NorthEast => match current_pos.1 {
                Position::East => (),
                Position::North => {
                    let p = current_pos.0.west();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                    let p = current_pos.0.south();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::NorthWest => match current_pos.1 {
                Position::North => (),
                Position::West => {
                    let p = current_pos.0.east();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                    let p = current_pos.0.south();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::SouthEast => match current_pos.1 {
                Position::South => (),
                Position::East => {
                    let p = current_pos.0.west();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                    let p = current_pos.0.north();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::SouthWest => match current_pos.1 {
                Position::West => (),
                Position::South => {
                    let p = current_pos.0.east();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                    let p = current_pos.0.north();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::Vertical => match current_pos.1 {
                Position::South => {
                    let p = current_pos.0.east();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                Position::North => {
                    let p = current_pos.0.west();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            Pipe::Horizontal => match current_pos.1 {
                Position::East => {
                    let p = current_pos.0.north();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                Position::West => {
                    let p = current_pos.0.south();
                    if !pipes_in_loop.contains(&p) {
                        if inside.insert(p) {
                            queue.push(p);
                        }
                    }
                }
                _ => panic!("Invalid position for pipe: {:?}", current_pos.1),
            },
            _ => panic!("Invalid pipe: {:?}", pipe),
        }

        let next_pos = next_pipe(*pipe, current_pos.1);
        let coord = match next_pos {
            Position::North => current_pos.0.north(),
            Position::South => current_pos.0.south(),
            Position::East => current_pos.0.east(),
            Position::West => current_pos.0.west(),
        };
        let entry_pos = next_pos.opposite();
        current_pos = (coord, entry_pos);
        if current_pos.0 == start_pos {
            break;
        }
    }

    // dbg!(&queue);
    // exit(1);

    while let Some(p) = queue.pop() {
        // dbg!(&queue.len());
        let adjacent_pipes = vec![p.north(), p.south(), p.west(), p.east()];
        for p in adjacent_pipes {
            if !pipes_in_loop.contains(&p) {
                if inside.insert(p) {
                    queue.push(p);
                }
            }
        }
    }

    (pipes_in_loop.len() / 2, inside.len())
}

fn main() {
    let file = aoc_util::read_input_file(2023, 10);
    let start = Instant::now();

    let mut map = Map2D::from_string(&file, Pipe::from_char);

    let (res1, res2) = discover_loop(&mut map);
    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
