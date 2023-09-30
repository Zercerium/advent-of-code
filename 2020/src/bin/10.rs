use std::collections::HashMap;

use petgraph::{prelude::GraphMap, Directed, Direction};

// Day 10: Adapter Array
fn main() {
    let file = aoc_util::read_input_file(2020, 10);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn parse(s: &str) -> Vec<u8> {
    let mut vec = vec![0];
    vec.extend(s.lines().map(|l| l.parse::<u8>().unwrap()));
    vec.sort();
    let l = vec.len();
    vec.dedup();
    assert_eq!(l, vec.len(), "Input contains duplicates");
    let device_input = vec.last().unwrap() + 3;
    vec.push(device_input);
    vec
}

fn solver1(s: &str) -> usize {
    let adapters = parse(s);
    let (one, three) = adapters
        .windows(2)
        .fold((0, 0), |acc, w| match w[1] - w[0] {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => acc,
        });
    one * three
}

fn solver2(s: &str) -> usize {
    let adapters = parse(s);
    let mut graph = GraphMap::<u8, (), Directed>::new();
    adapters.iter().for_each(|n| {
        graph.add_node(*n);
    });
    adapters.iter().for_each(|&n| {
        let possible_neighbors = (n + 1)..=(n + 3);
        possible_neighbors.for_each(|p| {
            if graph.contains_node(p) {
                graph.add_edge(n, p, ());
            }
        });
    });
    let mut in_edges_map = HashMap::new();
    // let graph = graph.into_graph();
    let mut topo = petgraph::visit::Topo::new(&graph);
    topo.next(&graph); // skip first node
    in_edges_map.insert(0, 1);
    while let Some(node) = topo.next(&graph) {
        // println!("Node: {}", node);
        let in_edges = graph
            .edges_directed(node, Direction::Incoming)
            .map(|e| in_edges_map[&e.0])
            .sum();
        in_edges_map.insert(node, in_edges);
    }
    *in_edges_map.get(adapters.last().unwrap()).unwrap()
}

#[cfg(test)]
mod test {
    use crate::*;

    const S: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_solver1() {
        let r = solver1(S);
        assert_eq!(r, 220);
    }

    #[test]
    fn test_solver2() {
        let r = solver2(S);
        assert_eq!(r, 19208);
    }
}
