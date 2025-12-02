use advent_of_code_2025::utils;
use std::time::Instant;

fn main() {
    let input = utils::read_single_line_separated("src/inputs/input-day2.txt", ",");

    let t = Instant::now();
    println!("Part 1: {} ({:?})", part1(&input), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {} ({:?})", part2(&input), t.elapsed());
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|range| {
            let (start, end) = parse_range(range);
            sum_even_invalid_ids_for_range(start, end)
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|range| {
            let (start, end) = parse_range(range);
            sum_all_invalid_ids_for_range(start, end)
        })
        .sum()
}

fn parse_range(range: &str) -> (usize, usize) {
    let (start, end) = range.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn sum_even_invalid_ids_for_range(start: usize, end: usize) -> usize {
    (start..=end)
        .filter(|&num| {
            let s = num.to_string();
            let length = s.len();
            length % 2 == 0 && s[..length / 2] == s[length / 2..]
        })
        .sum()
}

fn sum_all_invalid_ids_for_range(start: usize, end: usize) -> usize {
    (start..=end)
        .filter(|&num| {
            let s = num.to_string();
            let length = s.len();
            utils::find_divisors(length)
                .iter()
                .filter(|&&d| d < length)
                .any(|&d| {
                    let chunks: Vec<_> = s.as_bytes().chunks(d).collect();
                    chunks.windows(2).all(|w| w[0] == w[1])
                })
        })
        .sum()
}
