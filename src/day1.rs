use advent_of_code_2025::utils;
use std::time::Instant;

fn main() {
    let input = utils::read_lines("src/inputs/input-day1.txt");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part1(&input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part2(&input), t.elapsed());
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(starting_point, zero_count), current_rotation| {
            rotate_dial(starting_point, zero_count, current_rotation, false)
        })
        .1
}

fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(starting_point, zero_count), current_rotation| {
            rotate_dial(starting_point, zero_count, current_rotation, true)
        })
        .1
}

fn rotate_dial(
    starting_point: i32,
    zero_count: u32,
    current_rotation: &str,
    count_all_zero: bool,
) -> (i32, u32) {
    let (direction, offset) = current_rotation.split_at(1);
    let num_offset: i32 = offset.parse().unwrap();

    let new_starting_point = if direction == "R" {
        (starting_point + num_offset).rem_euclid(100)
    } else {
        (starting_point - num_offset).rem_euclid(100)
    };

    let zero_hits = if count_all_zero {
        count_all_zero_clicks(starting_point, num_offset, direction) as u32
    } else {
        if new_starting_point == 0 { 1 } else { 0 }
    };

    (new_starting_point, zero_count + zero_hits)
}

fn count_all_zero_clicks(starting_point: i32, num_offset: i32, direction: &str) -> i32 {
    let distance_to_zero = match (starting_point, direction) {
        (0, _) => 100,
        (_, "R") => 100 - starting_point,
        _ => starting_point,
    };

    if num_offset >= distance_to_zero {
        1 + (num_offset - distance_to_zero) / 100
    } else {
        0
    }
}
