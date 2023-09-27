use std::collections::HashSet;

// Day 1: Report Repair
fn main() {
    let file = aoc_util::read_input_file(2020, 1);
    let sum = 2020;

    // Part 1
    let mut map = HashSet::new();
    let res = file
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .filter_map(|n| {
            let difference = sum - n;
            if map.contains(&difference) {
                Some((n, difference))
            } else {
                map.insert(n);
                None
            }
        })
        .map(|(n, m)| n * m)
        .next();
    println!("Result Part 1: {:?}", res);

    // Part 2
    let mut i = file
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    i.sort_unstable();

    let mut res = None;
    'outer: for (idx, n) in i.iter().enumerate() {
        let mut f_p = idx + 1;
        let mut l_p = i.len() - 1;
        for _ in idx..i.len() - 1 {
            let s = n + i[f_p] + i[l_p];
            match s.cmp(&sum) {
                std::cmp::Ordering::Less => f_p += 1,
                std::cmp::Ordering::Greater => l_p -= 1,
                std::cmp::Ordering::Equal => {
                    res = Some(n * i[f_p] * i[l_p]);
                    break 'outer;
                }
            }
        }
    }
    println!("Result Part 2: {:?}", res)
}
