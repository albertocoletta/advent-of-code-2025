# Advent of Code 2025 - Task Runner

# Add a new day (usage: just new-day 2)
new-day day:
    #!/usr/bin/env bash
    set -euo pipefail
    
    DAY={{day}}
    
    # Validate input is a number
    if ! [[ "$DAY" =~ ^[0-9]+$ ]]; then
        echo "Error: Day must be a number"
        exit 1
    fi
    
    RUST_FILE="src/day${DAY}.rs"
    INPUT_FILE="src/inputs/input-day${DAY}.txt"
    
    # Check if files already exist
    if [[ -f "$RUST_FILE" ]]; then
        echo "Error: $RUST_FILE already exists"
        exit 1
    fi
    
    if [[ -f "$INPUT_FILE" ]]; then
        echo "Error: $INPUT_FILE already exists"
        exit 1
    fi
    
    # Create empty input file
    touch "$INPUT_FILE"
    echo "Created $INPUT_FILE"
    
    # Create Rust file from template
    printf '%s\n' \
        'use advent_of_code_2025::utils::*;' \
        'use std::time::Instant;' \
        '' \
        'fn main() {' \
        "    let input = read_file(\"src/inputs/input-day${DAY}.txt\");" \
        '' \
        '    let t = Instant::now();' \
        '    println!("Part 1: {} ({:?})", todo!(), t.elapsed());' \
        '' \
        '    let t = Instant::now();' \
        '    println!("Part 2: {} ({:?})", todo!(), t.elapsed());' \
        '}' > "$RUST_FILE"
    echo "Created $RUST_FILE"
    
    # Add bin entry to Cargo.toml
    printf '\n[[bin]]\nname = "day%s"\npath = "src/day%s.rs"\n' "$DAY" "$DAY" >> Cargo.toml
    echo "Added day${DAY} to Cargo.toml"
    
    echo ""
    echo "✓ Day ${DAY} created! Run with: cargo run --bin day${DAY}"

# Run a specific day (usage: just run 1)
run day:
    cargo run --bin day{{day}}

# Run a specific day in release mode
run-opt day:
    cargo run --release --bin day{{day}}
