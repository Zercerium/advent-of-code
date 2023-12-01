// Day 1: Trebuchet?!
fn main() {
    let file = aoc_util::read_input_file(2023, 1);

    // Part 1
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

    println!("Result Part 1: {:?}", res);

    // Part 2
    let res: u32 = file
        .lines()
        .map(|line| {
            // println!("{}", line);
            let mut first = None;
            let mut ll = line.to_string();
            while first.is_none() {
                first = find_first_number(&ll);
                ll = ll.chars().skip(1).collect::<String>();
                // println!("{:#?}", first)
            }

            let mut last = None;
            let mut ll = line.to_string();
            while last.is_none() {
                last = find_last_number(&ll);
                ll.pop();
            }

            // println!("first: {}, second: {}", first.unwrap(), last.unwrap());
            first.unwrap() * 10 + last.unwrap()
        })
        .sum();
    println!("Result Part 2: {:?}", res)
}

fn find_first_number(s: &str) -> Option<u32> {
    // println!("{}", s);
    let first = s.chars().next().unwrap().to_digit(10);
    if let Some(first) = first {
        return Some(first);
    }

    if s.starts_with("one") {
        return Some(1);
    }
    if s.starts_with("two") {
        return Some(2);
    }
    if s.starts_with("three") {
        return Some(3);
    }
    if s.starts_with("four") {
        return Some(4);
    }
    if s.starts_with("five") {
        return Some(5);
    }
    if s.starts_with("six") {
        return Some(6);
    }
    if s.starts_with("seven") {
        return Some(7);
    }
    if s.starts_with("eight") {
        return Some(8);
    }
    if s.starts_with("nine") {
        return Some(9);
    }
    None
}

fn find_last_number(s: &str) -> Option<u32> {
    let first = s.chars().rev().next().unwrap().to_digit(10);

    if let Some(first) = first {
        return Some(first);
    }

    if s.ends_with("one") {
        return Some(1);
    }
    if s.ends_with("two") {
        return Some(2);
    }
    if s.ends_with("three") {
        return Some(3);
    }
    if s.ends_with("four") {
        return Some(4);
    }
    if s.ends_with("five") {
        return Some(5);
    }
    if s.ends_with("six") {
        return Some(6);
    }
    if s.ends_with("seven") {
        return Some(7);
    }
    if s.ends_with("eight") {
        return Some(8);
    }
    if s.ends_with("nine") {
        return Some(9);
    }
    None
}
