use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// ============================================================================
// Common Grid Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Coordinate {
    pub line: i32,
    pub column: i32,
}

impl Coordinate {
    pub fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }

    pub fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            line: self.line + other.line,
            column: self.column + other.column,
        }
    }

    pub fn subtract(self, other: Coordinate) -> Coordinate {
        Coordinate {
            line: self.line - other.line,
            column: self.column - other.column,
        }
    }

    pub fn manhattan_distance(self, other: Coordinate) -> u32 {
        ((self.line - other.line).abs() + (self.column - other.column).abs()) as u32
    }

    /// Check if coordinate is within grid bounds
    pub fn is_in_bounds(self, grid_size: GridSize) -> bool {
        self.line >= 0
            && self.column >= 0
            && (self.line as usize) < grid_size.lines
            && (self.column as usize) < grid_size.columns
    }

    /// Get the coordinate to the left
    pub fn left(self) -> Coordinate {
        Coordinate::new(self.line, self.column - 1)
    }

    /// Get the coordinate to the right
    pub fn right(self) -> Coordinate {
        Coordinate::new(self.line, self.column + 1)
    }

    /// Get the coordinate above
    pub fn up(self) -> Coordinate {
        Coordinate::new(self.line - 1, self.column)
    }

    /// Get the coordinate below
    pub fn down(self) -> Coordinate {
        Coordinate::new(self.line + 1, self.column)
    }

    /// Get horizontal neighboring coordinates (left, right)
    pub fn horizontal_neighbors(self) -> [Coordinate; 2] {
        [self.left(), self.right()]
    }

    /// Get vertical neighboring coordinates (up, down)
    pub fn vertical_neighbors(self) -> [Coordinate; 2] {
        [self.up(), self.down()]
    }

    /// Get neighboring coordinates (up, down, left, right)
    pub fn neighbors_without_diagonals(self) -> [Coordinate; 4] {
        [
            Coordinate::new(self.line - 1, self.column), // up
            Coordinate::new(self.line + 1, self.column), // down
            Coordinate::new(self.line, self.column - 1), // left
            Coordinate::new(self.line, self.column + 1), // right
        ]
    }

    /// Get all 8 neighboring coordinates (including diagonals)
    pub fn neighbors_with_diagonals(self) -> [Coordinate; 8] {
        [
            Coordinate::new(self.line - 1, self.column),     // up
            Coordinate::new(self.line + 1, self.column),     // down
            Coordinate::new(self.line, self.column - 1),     // left
            Coordinate::new(self.line, self.column + 1),     // right
            Coordinate::new(self.line - 1, self.column - 1), // up-left
            Coordinate::new(self.line - 1, self.column + 1), // up-right
            Coordinate::new(self.line + 1, self.column - 1), // down-left
            Coordinate::new(self.line + 1, self.column + 1), // down-right
        ]
    }
}

/// Grid dimensions
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct GridSize {
    pub lines: usize,
    pub columns: usize,
}

impl GridSize {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self { lines, columns }
    }

    /// Check if a coordinate is within this grid
    pub fn contains(self, coord: Coordinate) -> bool {
        coord.is_in_bounds(self)
    }
}

// ============================================================================
// File Reading Utilities
// ============================================================================

/// Reads all lines from a file and returns them as a Vec<String>
///
/// # Example
/// ```
/// let lines = read_lines("src/inputs/input-day1.txt");
/// ```
pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect(&format!("Failed to open file: {}", path));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect()
}

/// Reads the entire file as a single string
///
/// # Example
/// ```
/// let content = read_file("src/inputs/input-day3.txt");
/// ```
pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect(&format!("Failed to read file: {}", path))
}

/// Reads file and splits by double newlines (useful for sections)
///
/// # Example
/// ```
/// let sections = read_sections("src/inputs/input-day5.txt");
/// // sections[0] contains first section, sections[1] contains second, etc.
/// ```
pub fn read_sections(path: &str) -> Vec<String> {
    read_file(path)
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

// ============================================================================
// String Parsing Utilities
// ============================================================================

/// Splits a string into lines and returns them as a Vec<String>
///
/// # Example
/// ```
/// let content = "line1\nline2\nline3";
/// let lines = lines_from_str(content);
/// assert_eq!(lines, vec!["line1", "line2", "line3"]);
/// ```
pub fn lines_from_str(content: &str) -> Vec<String> {
    content.lines().map(|s| s.to_string()).collect()
}

/// Splits a string by double newlines (useful for sections)
///
/// # Example
/// ```
/// let content = "section1\ndata1\n\nsection2\ndata2";
/// let sections = sections_from_str(content);
/// assert_eq!(sections.len(), 2);
/// ```
pub fn sections_from_str(content: &str) -> Vec<String> {
    content.split("\n\n").map(|s| s.to_string()).collect()
}

/// Parses a range string in the format "start-end" and returns a tuple of (start, end).
///
/// # Example
/// ```
/// let (start, end) = parse_range("1-5");
/// assert_eq!(start, 1);
/// assert_eq!(end, 5);
/// ```
pub fn parse_range(range: &str) -> (usize, usize) {
    let (start, end) = range.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

/// Reads lines and parses each line as whitespace-separated numbers
///
/// # Example
/// ```
/// let numbers: Vec<Vec<u32>> = read_lines_as_numbers("src/inputs/input-day2.txt");
/// ```
pub fn read_lines_as_numbers<T: FromStr>(path: &str) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    read_lines(path)
        .iter()
        .map(|line| parse_numbers(line))
        .collect()
}

/// Parses a line containing whitespace-separated numbers
///
/// # Example
/// ```
/// let numbers: Vec<u32> = parse_numbers("1 2 3 4 5");
/// ```
pub fn parse_numbers<T: FromStr>(line: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    line.split_whitespace()
        .map(|n| n.parse::<T>().expect("Failed to parse number"))
        .collect()
}

/// Reads a single line from a file and parses it as strings separated by a custom separator
///
/// # Example
/// ```
/// let items: Vec<String> = read_line_separated("src/inputs/input-day4.txt", ",");
/// ```
pub fn read_single_line_separated(path: &str, separator: &str) -> Vec<String> {
    let content = read_file(path);
    let first_line = content.lines().next().unwrap_or("");
    first_line
        .split(separator)
        .map(|s| s.trim().to_string())
        .collect()
}

// ============================================================================
// Grid Reading Utilities
// ============================================================================

/// Reads a grid of characters using Coordinate as keys
///
/// # Example
/// ```
/// let (grid, size) = read_char_grid("src/inputs/input-day6.txt");
/// let char_at = grid.get(&Coordinate::new(0, 0));
/// ```
pub fn read_char_grid(path: &str) -> (HashMap<Coordinate, char>, GridSize) {
    let lines = read_lines(path);
    let size = GridSize::new(
        lines.len(),
        if lines.is_empty() { 0 } else { lines[0].len() },
    );
    let mut grid = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert(Coordinate::new(row as i32, col as i32), ch);
        }
    }

    (grid, size)
}

/// Reads a grid of single-digit numbers using Coordinate as keys
///
/// # Example
/// ```
/// let (grid, size) = read_digit_grid("src/inputs/input-day10.txt");
/// let digit_at = grid.get(&Coordinate::new(0, 0));
/// ```
pub fn read_digit_grid(path: &str) -> (HashMap<Coordinate, u32>, GridSize) {
    let lines = read_lines(path);
    let size = GridSize::new(
        lines.len(),
        if lines.is_empty() { 0 } else { lines[0].len() },
    );
    let mut grid = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let digit = ch.to_digit(10).expect("Failed to parse digit");
            grid.insert(Coordinate::new(row as i32, col as i32), digit);
        }
    }

    (grid, size)
}

/// Gets the dimensions of a grid file
///
/// # Example
/// ```
/// let size = get_grid_size("src/inputs/input-day8.txt");
/// println!("Grid is {}x{}", size.lines, size.columns);
/// ```
pub fn get_grid_size(path: &str) -> GridSize {
    let lines = read_lines(path);
    GridSize::new(
        lines.len(),
        if lines.is_empty() { 0 } else { lines[0].len() },
    )
}

// ============================================================================
// General Utilities
// ============================================================================

pub fn find_divisors(num: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = Vec::new();
    let up_to = (num as f64).sqrt() as usize;
    for i in 1..=up_to {
        if num % i == 0 {
            divisors.push(i);
            if i != num / i {
                divisors.push(num / i);
            }
        }
    }
    divisors
}
