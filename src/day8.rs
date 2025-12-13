use advent_of_code_2025::utils;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
    b1: Cube,
    b2: Cube,
}

fn main() {
    let input = utils::read_lines("src/inputs/input-day8.txt");
    let boxes: Vec<Cube> = input
        .iter()
        .map(|line| {
            let numbers = utils::parse_numbers_sep(line, ",");
            Cube {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect();

    let t = Instant::now();
    let (part1, part2) = part1_and_2(&boxes, 10);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("({:?} for both parts)", t.elapsed())
}

fn part1_and_2(boxes: &[Cube], iterations: usize) -> (usize, isize) {
    let mut pairs_processed = 0;
    let mut components = boxes.len();
    let mut heap: BinaryHeap<Reverse<(isize, Pair)>> = BinaryHeap::new();
    let mut parent_map: HashMap<Cube, Cube> = HashMap::with_capacity(boxes.len());
    let mut size_map: HashMap<Cube, usize> = HashMap::with_capacity(boxes.len());
    let mut last_pair: Option<Pair> = None;
    let mut part1_answer: usize = 0;

    for b in boxes {
        parent_map.insert(*b, *b);
        size_map.insert(*b, 1);
    }

    for (b1, b2) in boxes.iter().tuple_combinations() {
        let d = squared_distance(b1, b2);
        heap.push(Reverse((d, Pair { b1: *b1, b2: *b2 })));
    }

    while let Some(el) = heap.pop() {
        let (_, pair) = el.0;
        let root1 = find(pair.b1, &parent_map);
        let root2 = find(pair.b2, &parent_map);
        if root1 != root2 {
            last_pair = Some(pair);
            let size1 = size_map[&root1];
            let size2 = size_map[&root2];
            if size1 >= size2 {
                parent_map.insert(root2, root1);
                *size_map.get_mut(&root1).unwrap() += size2;
            } else {
                parent_map.insert(root1, root2);
                *size_map.get_mut(&root2).unwrap() += size1;
            }
            components -= 1;
        }
        pairs_processed += 1;

        if pairs_processed == iterations {
            part1_answer = calculate_three_largest_circuits(&size_map, &parent_map);
        }

        if components == 1 {
            break;
        }
    }

    (
        part1_answer,
        last_pair.unwrap().b1.x * last_pair.unwrap().b2.x,
    )
}

fn calculate_three_largest_circuits(
    size_map: &HashMap<Cube, usize>,
    parent_map: &HashMap<Cube, Cube>,
) -> usize {
    size_map
        .iter()
        .filter(|(k, _)| is_root(k, parent_map))
        .map(|(_, &v)| v)
        .k_largest(3)
        .product()
}

fn find(b: Cube, parent_map: &HashMap<Cube, Cube>) -> Cube {
    let mut current = b;
    while parent_map[&current] != current {
        current = parent_map[&current];
    }
    current
}

fn is_root(cube: &Cube, parent_map: &HashMap<Cube, Cube>) -> bool {
    parent_map[cube] == *cube
}

fn squared_distance(box1: &Cube, box2: &Cube) -> isize {
    (box1.x - box2.x).pow(2) + (box1.y - box2.y).pow(2) + (box1.z - box2.z).pow(2)
}
