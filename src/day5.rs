use advent_of_code_2025::utils;
use std::{ops::RangeInclusive, time::Instant};

fn main() {
    let input = utils::read_file("src/inputs/input-day5.txt");
    let sections = utils::sections_from_str(&input);

    let fresh_ingredients = utils::lines_from_str(&sections[0]);
    let available_ingredients = utils::lines_from_str(&sections[1]);

    let t = Instant::now();
    println!(
        "Part 1: {:?} ({:?})",
        part1(&fresh_ingredients, &available_ingredients),
        t.elapsed()
    );

    let t = Instant::now();
    println!(
        "Part 2: {:?} ({:?})",
        part2(&fresh_ingredients),
        t.elapsed()
    );
}

fn part1(fresh_ingredients: &[String], available_ingredients: &[String]) -> usize {
    let fresh_ranges = get_fresh_ranges(fresh_ingredients);
    available_ingredients
        .iter()
        .filter(|&a| {
            let n: usize = a.parse().unwrap();
            is_ingredient_fresh(n, &fresh_ranges)
        })
        .count()
}

fn part2(fresh_ingredients: &[String]) -> usize {
    let mut fresh_ranges = get_fresh_ranges(fresh_ingredients);
    fresh_ranges.sort_by_key(|r| *r.start());

    let final_ranges = fresh_ranges
        .into_iter()
        .fold(Vec::new(), |mut acc, current| {
            match acc.last() {
                Some(last) if ranges_overlap(&current, last) => {
                    let new_range = merge_ranges(&current, &acc.pop().unwrap());
                    acc.push(new_range);
                }
                _ => acc.push(current),
            }
            acc
        });

    final_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn ranges_overlap(range_1: &RangeInclusive<usize>, range_2: &RangeInclusive<usize>) -> bool {
    range_2.start() <= range_1.end() && range_1.start() <= range_2.end()
}

fn merge_ranges(
    current_range: &RangeInclusive<usize>,
    other: &RangeInclusive<usize>,
) -> RangeInclusive<usize> {
    let start = *current_range.start().min(other.start());
    let end = *current_range.end().max(other.end());
    start..=end
}

fn get_fresh_ranges(fresh_ingredients: &[String]) -> Vec<RangeInclusive<usize>> {
    fresh_ingredients
        .iter()
        .map(|range| {
            let (start, end) = utils::parse_range(range);
            start..=end
        })
        .collect()
}

fn is_ingredient_fresh(ingredient: usize, fresh_ingredients: &[RangeInclusive<usize>]) -> bool {
    fresh_ingredients
        .iter()
        .any(|range| range.contains(&ingredient))
}
