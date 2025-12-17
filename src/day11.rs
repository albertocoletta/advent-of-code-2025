use advent_of_code_2025::utils;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    name: String,
    neighbors: Vec<String>,
}

fn main() {
    let input = utils::read_lines("src/inputs/input-day11-1.txt");
    let mut nodes: HashMap<String, Node> = input
        .iter()
        .filter_map(|s| s.split_once(": "))
        .map(|(name, neighbors)| Node {
            name: name.to_string(),
            neighbors: neighbors.split(' ').map(String::from).collect(),
        })
        .map(|n| (n.name.clone(), n))
        .collect();
    nodes.insert(
        "out".into(),
        Node {
            name: "out".into(),
            neighbors: vec![],
        },
    );

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&nodes), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {:?} ({:?})", part2(&nodes), t.elapsed());
}

fn part1(nodes: &HashMap<String, Node>) -> usize {
    let start = nodes.get("you").unwrap().clone();
    count_paths(start, |n| get_successors(n, nodes), |n| n.name == "out")
}

fn part2(nodes: &HashMap<String, Node>) -> usize {
    let dac_first = count_paths_between(nodes, "svr", "dac")
        * count_paths_between(nodes, "dac", "fft")
        * count_paths_between(nodes, "fft", "out");

    let fft_first = count_paths_between(nodes, "svr", "fft")
        * count_paths_between(nodes, "fft", "dac")
        * count_paths_between(nodes, "dac", "out");

    dac_first + fft_first
}

fn count_paths_between(nodes: &HashMap<String, Node>, from: &str, to: &str) -> usize {
    let start = nodes.get(from).unwrap().clone();
    count_paths(start, |n| get_successors(n, nodes), |n| n.name == to)
}

fn get_successors(n: &Node, nodes: &HashMap<String, Node>) -> Vec<Node> {
    n.neighbors
        .iter()
        .filter_map(|name| nodes.get(name.as_str()).cloned())
        .collect()
}
