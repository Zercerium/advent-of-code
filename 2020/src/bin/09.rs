use std::collections::{HashMap, VecDeque};

// Day 9: Encoding Error
fn main() {
    let file = aoc_util::read_input_file(2020, 9);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn parse(s: &str) -> Vec<usize> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn solver1(s: &str) -> usize {
    let numbers = parse(s);
    let mut buffer = VecDeque::with_capacity(25);
    let mut number_map = HashMap::new();
    numbers[0..25].iter().for_each(|n| {
        buffer.iter().for_each(|e| {
            *number_map.entry(n + e).or_insert(0) += 1;
        });
        buffer.push_back(*n);
    });

    for (_, n) in numbers.into_iter().enumerate().skip(25) {
        if !number_map.contains_key(&n) {
            return n as usize;
        }
        let old = buffer.pop_front().unwrap();
        buffer.iter().for_each(|e| {
            // would also be possible to use an vec with the created elements instead to recalculate
            let entry = number_map.entry(old + e).or_insert(0);
            if *entry == 1 {
                number_map.remove(&(old + e));
            } else {
                *entry -= 1;
            }
        });
        buffer.iter().for_each(|e| {
            *number_map.entry(n + e).or_insert(0) += 1;
        });
        buffer.push_back(n);
    }
    0
}

fn solver2(s: &str) -> usize {
    let numbers = parse(s);
    let invalid = solver1(s);
    let mut start = 0;
    let mut end = 1;
    let mut sum = numbers[0] + numbers[1];
    while sum != invalid {
        if sum < invalid {
            end += 1;
            sum += numbers[end];
        } else {
            sum -= numbers[start];
            start += 1;
        }
    }
    // would be interesting if this runs 2x times or searches for the min/max at the same time
    numbers[start..=end].iter().min().unwrap() + numbers[start..=end].iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_solver1() {
        let s = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let r = solver1(s);
        assert_eq!(r, 5);
    }

    #[test]
    fn test_solver2() {
        let s = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let r = solver2(s);
        assert_eq!(r, 8);
    }
}
