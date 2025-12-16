use advent_of_code_2025::utils;
use pathfinding::prelude::bfs;
use regex::Regex;
use std::sync::LazyLock;
use std::time::Instant;

static MACHINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]|\(([^)]+)\)|\{([^}]+)\}").unwrap());

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

fn main() {
    let input = utils::read_lines("src/inputs/input-day10.txt");
    let machines: Vec<Machine> = input.iter().map(|s| parse_raw_machine(s)).collect();

    let t = Instant::now();
    println!("Part 1: {:?} ({:?})", part1(&machines), t.elapsed());
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|m| find_fewest_button_presses(&m.lights, &m.buttons))
        .sum()
}

fn find_fewest_button_presses(lights: &[bool], buttons: &[Vec<usize>]) -> usize {
    let s: Vec<bool> = vec![false; lights.len()];

    bfs(
        &s,
        |l| get_successors(l, buttons),
        |final_lights| final_lights == &lights,
    )
    .unwrap()
    .len()
        - 1
}

fn get_successors(current_lights: &[bool], buttons: &[Vec<usize>]) -> Vec<Vec<bool>> {
    buttons
        .iter()
        .map(|indices| {
            let mut new_lights = current_lights.to_vec();
            for &idx in indices {
                new_lights[idx] = !new_lights[idx];
            }
            new_lights
        })
        .collect()
}

fn parse_raw_machine(line: &str) -> Machine {
    let mut lights = String::new();
    let mut buttons = Vec::new();
    let mut joltage_requirements = String::new();

    for caps in MACHINE_RE.captures_iter(line) {
        match (caps.get(1), caps.get(2), caps.get(3)) {
            (Some(m), _, _) => lights = m.as_str().to_string(),
            (_, Some(m), _) => buttons.push(m.as_str().to_string()),
            (_, _, Some(m)) => joltage_requirements = m.as_str().to_string(),
            _ => {}
        }
    }

    let bool_lights: Vec<bool> = lights.chars().map(|c| c != '.').collect();

    let num_joltage: Vec<usize> = joltage_requirements
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let parsed_buttons: Vec<Vec<usize>> = buttons
        .iter()
        .map(|b| b.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    Machine {
        lights: bool_lights,
        buttons: parsed_buttons,
        joltage_requirements: num_joltage,
    }
}
