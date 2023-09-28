use aoc_util::map::{Map2D, Point2D};

// Day 3: Toboggan Trajectory
fn main() {
    let file = aoc_util::read_input_file(2020, 3);
    let map = parse(&file);
    // Part 1
    let slope = (3, 1);
    let trees = count_trees(&map, slope);
    println!("Result Part 1: {:?}", trees);

    // Part 2
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees = slopes
        .iter()
        .map(|s| count_trees(&map, *s))
        .product::<usize>();
    println!("Result Part 2: {:?}", trees);
}

fn count_trees(map: &Map2D<MapItem>, slope: (u16, u16)) -> usize {
    let mut location = Point2D::<u16>::new(0, 0);
    let mut trees = 0;

    location.add(slope);
    while location.y < map.columns() as u16 {
        if *map.get(location) == MapItem::Tree {
            trees += 1;
        }
        location.add(slope);
    }
    trees
}

fn parse(s: &str) -> Map2D<MapItem> {
    let length = s.lines().next().unwrap().len();
    let mut map = Map2D::new(length);
    let f = |s: &str| -> Vec<MapItem> {
        s.chars()
            .map(|c| match c {
                '.' => MapItem::Nothing,
                '#' => MapItem::Tree,
                _ => unreachable!(),
            })
            .collect()
    };
    s.lines().for_each(|s| map.append_row(&mut f(s)));
    map.set_x_torus();
    map
}

#[derive(Clone, PartialEq, Eq)]
enum MapItem {
    Tree,
    Nothing,
}
