use advent_of_code_2025::utils;
use advent_of_code_2025::utils::Coordinate;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = utils::read_lines("src/inputs/input-day7.txt");

    let t = Instant::now();
    let (used_splitters, timelines) = part1_and_2(&input);
    println!(
        "Part 1: {:?} \nPart 2: {:?} \n({:?} for both parts)",
        used_splitters,
        timelines,
        t.elapsed()
    );
}

fn part1_and_2(input: &[String]) -> (usize, usize) {
    let mut used_splitters: HashSet<Coordinate> = HashSet::new();
    let mut counts: HashMap<Coordinate, usize> = HashMap::new();
    for (line_idx, line) in input.iter().enumerate() {
        for (col_idx, ch) in line.char_indices() {
            let current_coord = Coordinate::new(line_idx as i32, col_idx as i32);
            match ch {
                'S' => {
                    let new_beam = current_coord.down();
                    counts.insert(new_beam, 1);
                }
                '^' => {
                    if let Some(&count_above) = counts.get(&current_coord.up()) {
                        let left = current_coord.left();
                        let right = current_coord.right();
                        for neighbor in [left, right] {
                            *counts.entry(neighbor).or_insert(0) += count_above;
                        }

                        used_splitters.insert(current_coord);
                    }
                }
                _ => {
                    if let Some(&count_above) = counts.get(&current_coord.up()) {
                        *counts.entry(current_coord).or_insert(0) += count_above;
                    }
                }
            }
        }
    }

    let last_row = (input.len() - 1) as i32;
    let total_timelines: usize = counts
        .iter()
        .filter(|(coord, _)| coord.line == last_row)
        .map(|(_, &count)| count)
        .sum();

    (used_splitters.len(), total_timelines)
}
