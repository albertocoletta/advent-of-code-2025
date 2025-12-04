use advent_of_code_2025::utils;
use advent_of_code_2025::utils::Coordinate;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let (grid, _) = utils::read_char_grid("src/inputs/input-day4.txt");

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&grid), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {:?} ({:?})", part2(grid), t.elapsed());
}

fn part1(grid: &HashMap<Coordinate, char>) -> usize {
    identify_removable_rolls(grid).len()
}

fn part2(mut grid: HashMap<Coordinate, char>) -> usize {
    let mut total_rolls = 0;
    loop {
        let removable_rolls = identify_removable_rolls(&grid);
        if removable_rolls.is_empty() {
            break total_rolls;
        }
        total_rolls += removable_rolls.len();
        for coord in removable_rolls {
            grid.remove(&coord);
        }
    }
}

fn identify_removable_rolls(grid: &HashMap<Coordinate, char>) -> Vec<Coordinate> {
    grid.iter()
        .filter(|(k, v)| *v == &'@' && is_roll_paper_reachable(grid, &k.neighbors_with_diagonals()))
        .map(|(k, _)| *k)
        .collect()
}

fn is_roll_paper_reachable(grid: &HashMap<Coordinate, char>, neighbors: &[Coordinate]) -> bool {
    neighbors
        .iter()
        .filter(|n| grid.get(n).is_some_and(|c| *c == '@'))
        .count()
        < 4
}
