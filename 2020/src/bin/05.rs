// Day 5: Binary Boarding
fn main() {
    let file = aoc_util::read_input_file(2020, 5);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn solver1(s: &str) -> usize {
    s.lines().map(seat_id).max().unwrap()
}

fn seat_id(s: &str) -> usize {
    let row = cutter(&s.as_bytes()[0..7], 0, 127, 'F', 'B') as usize;
    let column = cutter(&s.as_bytes()[7..10], 0, 8, 'L', 'R') as usize;
    row * 8 + column
}

fn cutter(s: &[u8], mut min: u8, mut max: u8, l: char, u: char) -> u8 {
    for &c in s {
        let half = (max - min + 1) / 2;
        if c == l as u8 {
            max -= half;
        } else if c == u as u8 {
            min += half;
        } else {
            unreachable!();
        }
    }
    min
}

fn solver2(s: &str) -> usize {
    let mut ids: Vec<_> = s.lines().map(seat_id).collect();
    ids.sort();
    ids.windows(2)
        .find_map(|a| {
            if a[0] + 1 != a[1] {
                Some(a[0] + 1)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::seat_id;

    #[test]
    fn test_solver1() {
        let s = "FBFBBFFRLR";
        let a = seat_id(s);
        assert_eq!(a, 357);
    }
}
