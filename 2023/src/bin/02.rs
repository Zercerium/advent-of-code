// Day 2: Cube Conundrum

#[derive(Debug)]
struct Game {
    id: u32,
    views: Vec<View>,
}

#[derive(Debug)]
struct View {
    blue: u32,
    green: u32,
    red: u32,
}

fn parse(s: &str) -> Vec<Game> {
    s.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Game {
    let (id_str, subsets) = line.split_once(":").unwrap();
    let id: u32 = id_str.split_once(" ").unwrap().1.parse().unwrap();
    let views = parse_subsets(subsets);
    Game { id, views }
}

fn parse_subsets(s: &str) -> Vec<View> {
    s.split(";").map(|s| s.trim()).map(parse_subset).collect()
}

fn parse_subset(s: &str) -> View {
    let colors = s.split(",").map(|s| s.trim()).collect::<Vec<_>>();
    let colors = colors
        .iter()
        .map(|s| s.split_once(" ").unwrap())
        .collect::<Vec<_>>();
    let mut view = View {
        blue: 0,
        green: 0,
        red: 0,
    };
    colors.iter().for_each(|c| match c.1 {
        "blue" => view.blue = c.0.parse().unwrap(),
        "green" => view.green = c.0.parse().unwrap(),
        "red" => view.red = c.0.parse().unwrap(),
        _ => panic!("Unknown color"),
    });
    view
}

fn game_valid(game: &Game) -> Option<u32> {
    let valid = game
        .views
        .iter()
        .find(|v| v.blue > 14 || v.green > 13 || v.red > 12);
    if valid.is_none() {
        Some(game.id)
    } else {
        None
    }
}

fn min_possible(game: &Game) -> u32 {
    let mut view = View {
        blue: 0,
        green: 0,
        red: 0,
    };
    game.views.iter().for_each(|v| {
        if v.blue > view.blue {
            view.blue = v.blue;
        };
        if v.green > view.green {
            view.green = v.green;
        };
        if v.red > view.red {
            view.red = v.red;
        };
    });
    view.blue * view.green * view.red
}

fn main() {
    let file = aoc_util::read_input_file(2023, 2);
    let input = parse(&file);
    // println!("{:#?}", input);
    // Part 1
    let res: u32 = input.iter().filter_map(game_valid).sum();
    println!("{:#?}", res);

    // Part 2
    let res: u32 = input.iter().map(min_possible).sum();
    println!("{:#?}", res);
}
