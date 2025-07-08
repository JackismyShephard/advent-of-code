//! Day 4: Ceres Search
//!
//! Solution for Advent of Code 2024 Day 4.
//!
//! Part 1: Find all occurrences of "XMAS" in a 2D word search grid.
//! The word can appear horizontally, vertically, or diagonally, and can be
//! written forwards or backwards.

use anyhow::Result;
use shared::input::parse_lines;

/// Example input from the problem statement used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

/// Direction vectors for all 8 possible directions in the grid.
/// Format: (row_delta, col_delta) where each delta is -1, 0, or 1
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // right
    (0, -1),  // left
    (1, 0),   // down
    (-1, 0),  // up
    (1, 1),   // down-right
    (-1, -1), // up-left
    (1, -1),  // down-left
    (-1, 1),  // up-right
];

/// The target word we're searching for.
const TARGET: &str = "XMAS";

/// Parses the input string into a 2D grid of characters.
///
/// Takes the input text and converts it into a vector of character vectors,
/// where each inner vector represents a row in the grid.
///
/// # Parameters
/// * `input` - Multi-line string containing the character grid
///
/// # Returns
/// 2D vector of characters representing the grid
///
/// # Examples
///
/// ```
/// # use day04::parse_input;
/// let input = "ABC\nDEF";
/// let grid = parse_input(input);
/// assert_eq!(grid, vec![
///     vec!['A', 'B', 'C'],
///     vec!['D', 'E', 'F']
/// ]);
/// ```
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse_lines(input)
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

/// Checks if the target word appears in a specific direction from a given position.
///
/// Starting from the given position, checks if the characters in the specified
/// direction match the target word "XMAS". Handles bounds checking to ensure
/// we don't go outside the grid.
///
/// # Parameters
/// * `grid` - The 2D character grid to search in
/// * `start_row` - Starting row position (0-indexed)
/// * `start_col` - Starting column position (0-indexed)
/// * `row_delta` - Row direction (-1, 0, or 1)
/// * `col_delta` - Column direction (-1, 0, or 1)
///
/// # Returns
/// `true` if the target word is found in the specified direction, `false` otherwise
///
/// # Examples
///
/// ```
/// # use day04::{parse_input, check_direction};
/// let grid = parse_input("XMAS\nABCD");
/// assert!(check_direction(&grid, 0, 0, 0, 1)); // "XMAS" going right
/// assert!(!check_direction(&grid, 0, 0, 1, 0)); // "XABC" going down
/// ```
pub fn check_direction(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    row_delta: isize,
    col_delta: isize,
) -> bool {
    let target_chars: Vec<char> = TARGET.chars().collect();

    for (i, &target_char) in target_chars.iter().enumerate() {
        let new_row = start_row as isize + (i as isize * row_delta);
        let new_col = start_col as isize + (i as isize * col_delta);

        // Check bounds
        if new_row < 0 || new_col < 0 {
            return false;
        }

        let row = new_row as usize;
        let col = new_col as usize;

        if row >= grid.len() || col >= grid[row].len() {
            return false;
        }

        // Check character match
        if grid[row][col] != target_char {
            return false;
        }
    }

    true
}

/// Counts the number of times "XMAS" appears starting from a specific position.
///
/// Checks all 8 directions from the given position and counts how many times
/// the target word "XMAS" appears.
///
/// # Parameters
/// * `grid` - The 2D character grid to search in
/// * `row` - Row position to start searching from (0-indexed)
/// * `col` - Column position to start searching from (0-indexed)
///
/// # Returns
/// Number of times "XMAS" appears starting from this position (0-8)
///
/// # Examples
///
/// ```
/// # use day04::{parse_input, count_xmas_at_position};
/// let grid = parse_input("XMAS\nMASX");
/// assert_eq!(count_xmas_at_position(&grid, 0, 0), 1); // "XMAS" going right
/// ```
pub fn count_xmas_at_position(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&(row_delta, col_delta)| check_direction(grid, row, col, row_delta, col_delta))
        .count()
}

/// Solves Part 1: Finds all occurrences of "XMAS" in the word search grid.
///
/// Searches through every position in the grid and counts how many times
/// "XMAS" appears in all 8 directions (horizontal, vertical, and diagonal).
/// Words can be written forwards or backwards.
///
/// # Parameters
/// * `input` - Multi-line string containing the character grid
///
/// # Returns
/// Total number of "XMAS" occurrences found in the grid
///
/// # Errors
///
/// Returns `Err` if the input cannot be parsed.
///
/// # Examples
///
/// ```
/// # use day04::solve_part1;
/// let input = "XMAS\nMASX";
/// assert_eq!(solve_part1(input).unwrap(), 1); // "XMAS" going right from (0,0)
/// ```
pub fn solve_part1(input: &str) -> Result<usize> {
    let grid = parse_input(input);

    let mut total_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            total_count += count_xmas_at_position(&grid, row, col);
        }
    }

    Ok(total_count)
}
