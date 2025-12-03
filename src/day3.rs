use advent_of_code_2025::utils;
use std::time::Instant;

fn main() {
    let input = utils::read_lines("src/inputs/input-day3.txt");

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {:?} ({:?})", part2(&input), t.elapsed());
}

fn part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|battery_bank| get_highest_voltage_per_bank(battery_bank, 2))
        .sum()
}

fn part2(input: &[String]) -> u64 {
    input
        .iter()
        .map(|battery_bank| get_highest_voltage_per_bank(battery_bank, 12))
        .sum()
}

fn get_highest_voltage_per_bank(battery_bank: &str, sequence_length: usize) -> u64 {
    let digits: Vec<u64> = battery_bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut stack: Vec<u64> = Vec::with_capacity(sequence_length);
    let mut digits_remaining = digits.len();

    for d in digits {
        while stack.last().is_some_and(|&last| last < d)
            && digits_remaining + stack.len() > sequence_length
        {
            stack.pop();
        }

        if stack.len() < sequence_length {
            stack.push(d);
        }
        digits_remaining -= 1;
    }

    stack.iter().fold(0, |acc, &d| acc * 10 + d)
}
