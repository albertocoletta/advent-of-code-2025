use advent_of_code_2025::utils::{self, Coordinate};
use itertools::Itertools;
use std::time::Instant;

struct VerticalEdge {
    x: isize,
    y_min: isize,
    y_max: isize,
}

struct HorizontalEdge {
    y: isize,
    x_min: isize,
    x_max: isize,
}

fn main() {
    let input = utils::read_lines("src/inputs/input-day9.txt");
    let coordinates: Vec<_> = input
        .iter()
        .map(|s| {
            let v: Vec<usize> = utils::parse_numbers_sep(&s, ",");
            Coordinate::new(v[0].try_into().unwrap(), v[1].try_into().unwrap())
        })
        .collect();

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&coordinates), t.elapsed());

    let t = Instant::now();
    println!("Part 2: {:?} ({:?})", part2(&coordinates), t.elapsed());
}

fn part1(input: &[Coordinate]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .map(|(c1, c2)| {
            ((c1.row - c2.row).abs() as usize + 1) * ((c1.column - c2.column).abs() as usize + 1)
        })
        .max()
        .unwrap()
}

fn part2(input: &[Coordinate]) -> usize {
    let (vertical_edges, horizontal_edges) = build_edges(input);

    input
        .iter()
        .tuple_combinations()
        .filter(|(c1, c2)| is_valid_rectangle(c1, c2, &vertical_edges, &horizontal_edges))
        .map(|(c1, c2)| {
            ((c1.row - c2.row).abs() as usize + 1) * ((c1.column - c2.column).abs() as usize + 1)
        })
        .max()
        .unwrap()
}

fn build_edges(coords: &[Coordinate]) -> (Vec<VerticalEdge>, Vec<HorizontalEdge>) {
    let mut vertical_edges = Vec::new();
    let mut horizontal_edges = Vec::new();

    for i in 0..coords.len() {
        let c1 = &coords[i];
        let c2 = &coords[(i + 1) % coords.len()];

        if c1.row == c2.row {
            horizontal_edges.push(HorizontalEdge {
                y: c1.row,
                x_min: c1.column.min(c2.column),
                x_max: c1.column.max(c2.column),
            });
        } else {
            vertical_edges.push(VerticalEdge {
                x: c1.column,
                y_min: c1.row.min(c2.row),
                y_max: c1.row.max(c2.row),
            });
        }
    }

    (vertical_edges, horizontal_edges)
}

fn ranges_overlap(a_min: isize, a_max: isize, b_min: isize, b_max: isize) -> bool {
    a_min < b_max && b_min < a_max
}

fn is_valid_rectangle(
    c1: &Coordinate,
    c2: &Coordinate,
    vertical_edges: &[VerticalEdge],
    horizontal_edges: &[HorizontalEdge],
) -> bool {
    let x_min = c1.column.min(c2.column);
    let x_max = c1.column.max(c2.column);
    let y_min = c1.row.min(c2.row);
    let y_max = c1.row.max(c2.row);

    let vertical_cuts = vertical_edges
        .iter()
        .any(|e| x_min < e.x && e.x < x_max && ranges_overlap(e.y_min, e.y_max, y_min, y_max));

    let horizontal_cuts = horizontal_edges
        .iter()
        .any(|e| y_min < e.y && e.y < y_max && ranges_overlap(e.x_min, e.x_max, x_min, x_max));

    !vertical_cuts && !horizontal_cuts
}
