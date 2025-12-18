use advent_of_code_2025::utils;
use std::{time::Instant, usize};

fn main() {
    let input = utils::read_lines("src/inputs/input-day12.txt");

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&input), t.elapsed());
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .filter(|s| {
            let (size, gifts) = s.split_once(": ").unwrap();
            let gifts_size: usize = gifts
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .sum();
            let (n1, n2) = size.split_once('x').unwrap();
            let grid_size = n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap();
            grid_size >= gifts_size * 9
        })
        .count()
}
