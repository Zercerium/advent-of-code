use std::collections::{HashMap, HashSet};

use petgraph::{visit::DfsPostOrder, Direction, Graph};
use regex::Regex;

// Day 7: Handy Haversacks
fn main() {
    let file = aoc_util::read_input_file(2020, 7);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn solver1(s: &str) -> usize {
    let different_bags = s.lines().count();
    let mut graph = Graph::<String, usize>::with_capacity(different_bags, different_bags * 6);
    let mut nodes = HashMap::new();
    s.lines().for_each(|line| {
        let regex = Regex::new(r"(?-u:\w)+ (?-u:\w)+").unwrap();
        let bag_color = regex.find(line).unwrap().as_str();
        let bag_index = nodes
            .entry(bag_color.to_string())
            .or_insert_with_key(|key| graph.add_node(key.to_string()))
            .clone();
        let regex = Regex::new(r"((?-u:\d)+) ((?-u:\w)+ (?-u:\w)+)").unwrap();
        regex
            .captures_iter(line)
            .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
            .for_each(|(count, color)| {
                let color_index = nodes
                    .entry(color)
                    .or_insert_with_key(|key| graph.add_node(key.to_string()));
                graph.add_edge(bag_index, *color_index, count);
            });
    });
    let shiny_gold_index = nodes.get("shiny gold").unwrap();
    let mut queue = vec![*shiny_gold_index];
    let mut set = HashSet::new();
    while let Some(node) = queue.pop() {
        let neighbors = graph.neighbors_directed(node, Direction::Incoming);
        neighbors.for_each(|n| {
            set.insert(n);
            queue.push(n);
        });
    }
    set.len()
}

fn solver2(s: &str) -> usize {
    let different_bags = s.lines().count();
    let mut graph = Graph::<usize, usize>::with_capacity(different_bags, different_bags * 6);
    let mut nodes = HashMap::new();
    s.lines().for_each(|line| {
        let regex = Regex::new(r"(?-u:\w)+ (?-u:\w)+").unwrap();
        let bag_color = regex.find(line).unwrap().as_str();
        let bag_index = nodes
            .entry(bag_color.to_string())
            .or_insert(graph.add_node(0))
            .clone();
        let regex = Regex::new(r"((?-u:\d)+) ((?-u:\w)+ (?-u:\w)+)").unwrap();
        regex
            .captures_iter(line)
            .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
            .for_each(|(count, color)| {
                let color_index = nodes.entry(color).or_insert(graph.add_node(0));
                graph.add_edge(bag_index, *color_index, count);
            });
    });
    let shiny_gold_index = nodes.get("shiny gold").unwrap();
    let mut dfs = DfsPostOrder::new(&graph, *shiny_gold_index);
    while let Some(nx) = dfs.next(&graph) {
        let neighbors = graph.neighbors(nx);
        if neighbors.count() == 0 {
            graph[nx] = 1;
        } else {
            let mut sum = 1;
            graph.neighbors(nx).for_each(|n| {
                let weight = graph.edge_weight(graph.find_edge(nx, n).unwrap()).unwrap();
                println!(
                    "{} {} {}",
                    graph[n],
                    weight,
                    nodes.iter().filter(|(_, v)| **v == n).next().unwrap().0
                );
                sum += graph[n] * weight;
            });
            graph[nx] = sum;
        }
    }
    graph[*shiny_gold_index] - 1
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_solver1() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(4, solver1(s));
        assert_eq!(32, solver2(s));
    }
}
