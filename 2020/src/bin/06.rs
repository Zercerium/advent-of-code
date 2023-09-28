use std::collections::HashSet;

fn main() {
    let file = aoc_util::read_input_file(2020, 6);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn solver1(s: &str) -> usize {
    s.split("\n\n").map(count).sum()
}

fn count(s: &str) -> usize {
    let mut set = HashSet::new();
    s.chars().filter(|&c| c != '\n').for_each(|c| {
        set.insert(c);
    });
    set.len()
}

fn solver2(s: &str) -> usize {
    s.split("\n\n").map(count2).sum()
}

fn count2(s: &str) -> usize {
    let mut vec = vec![0; 26];
    s.chars()
        .filter(|&c| c != '\n')
        .for_each(|c| vec[(c as u8 - b'a') as usize] += 1);
    let ppl = s.lines().count();
    vec.iter().filter(|&&n| n == ppl).count()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_solver1() {
        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let r = solver1(s);
        assert_eq!(r, 11);
    }

    #[test]
    fn test_solver2() {
        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let r = solver2(s);
        assert_eq!(r, 6);
    }
}
