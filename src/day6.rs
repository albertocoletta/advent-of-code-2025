use advent_of_code_2025::utils;
use itertools::izip;
use std::time::Instant;

fn main() {
    let mut input = utils::read_lines("src/inputs/input-day6.txt");
    let operations = input.pop().unwrap();
    let split_operations: Vec<_> = operations.split_ascii_whitespace().map(|s| s).collect();

    let t = Instant::now();
    println!(
        "Part 1: {:?} ({:?})",
        part1(&input, &split_operations),
        t.elapsed()
    );

    let t = Instant::now();
    println!(
        "Part 2: {:?} ({:?})",
        part2(&input, &split_operations),
        t.elapsed()
    );
}

fn part1(input: &[String], operations: &[&str]) -> usize {
    let numbers: Vec<Vec<&str>> = input
        .iter()
        .map(|v| v.split_ascii_whitespace().collect())
        .collect();
    izip!(
        &numbers[0],
        &numbers[1],
        &numbers[2],
        &numbers[3],
        operations
    )
    .map(|(x, y, z, k, op)| calculate(x, y, z, k, op))
    .sum()
}

fn part2(input: &[String], operations: &[&str]) -> usize {
    let spaced_numbers = extract_spaced_numbers(input);
    izip!(
        &spaced_numbers[0],
        &spaced_numbers[1],
        &spaced_numbers[2],
        &spaced_numbers[3],
        operations
    )
    .map(|(x, y, z, k, op)| calculate_part_2(x, y, z, k, op))
    .sum()
}

fn calculate(x: &str, y: &str, z: &str, k: &str, op: &str) -> usize {
    let nums: Vec<usize> = [x, y, z, k].iter().map(|s| s.parse().unwrap()).collect();

    if op == "+" {
        nums.iter().sum()
    } else {
        nums.iter().product()
    }
}

fn calculate_part_2(x: &str, y: &str, z: &str, k: &str, op: &str) -> usize {
    let x_chars: Vec<_> = x.chars().collect();
    let y_chars: Vec<_> = y.chars().collect();
    let z_chars: Vec<_> = z.chars().collect();
    let k_chars: Vec<_> = k.chars().collect();

    let s: Vec<_> = izip!(x_chars, y_chars, z_chars, k_chars)
        .map(|(x, y, z, k)| generate_vertical_number(x, y, z, k))
        .collect();

    if op == "+" {
        s.iter().sum()
    } else {
        s.iter().product()
    }
}

fn generate_vertical_number(x: char, y: char, z: char, k: char) -> usize {
    let nums = [
        x.to_digit(10),
        y.to_digit(10),
        z.to_digit(10),
        k.to_digit(10),
    ];
    nums.into_iter()
        .flatten()
        .fold(0, |acc, d| acc * 10 + d as usize)
}

fn extract_spaced_numbers(input: &[String]) -> Vec<Vec<String>> {
    let len = input[0].len();
    let mut result = vec![Vec::new(); 4];
    let mut problem_start_index: Option<usize> = None;

    for col in 0..=len {
        let is_separator = col == len || (0..4).all(|row| input[row].as_bytes()[col] == b' ');

        match problem_start_index {
            Some(start) if is_separator => {
                for row in 0..4 {
                    result[row].push(input[row][start..col].to_string());
                }
                problem_start_index = None;
            }
            None if !is_separator => {
                problem_start_index = Some(col);
            }
            _ => {}
        }
    }

    result
}
