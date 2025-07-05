//! Day 4: Ceres Search
//!
//! Solution for Advent of Code 2024 Day 4.
//!
//! Part 1: Find all occurrences of "XMAS" in a word search grid.
//! The word can appear horizontally, vertically, diagonally, forwards, or backwards.

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

/// Parses the input string into a 2D grid of characters.
///
/// Takes input with one row per line and converts it into a grid representation
/// optimized for fast access and pattern matching.
///
/// # Parameters
/// * `input` - Multi-line string with characters representing the word search grid
///
/// # Returns
/// A 2D vector of bytes representing the grid, where each byte is a character
///
/// # Examples
///
/// ```
/// # use day04::parse_input;
/// let input = "ABC\nDEF";
/// let grid = parse_input(input);
/// assert_eq!(grid.len(), 2);
/// assert_eq!(grid[0], b"ABC");
/// assert_eq!(grid[1], b"DEF");
/// ```
pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let lines = parse_lines(input);
    lines.iter().map(|line| line.bytes().collect()).collect()
}

/// Solves Part 1: Counts all occurrences of "XMAS" in the word search grid.
///
/// Searches for "XMAS" in all 8 directions: horizontal (left-right, right-left),
/// vertical (up-down, down-up), and diagonal (4 directions). Uses optimized
/// byte string matching for maximum performance.
///
/// # Parameters
/// * `input` - Multi-line string containing the word search grid
///
/// # Returns
/// Total count of "XMAS" occurrences found in any direction
///
/// # Errors
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day04::solve_part1;
/// let input = "XMAS\nSAMX";
/// assert_eq!(solve_part1(input).unwrap(), 2); // One forward, one backward
/// ```
pub fn solve_part1(input: &str) -> Result<i32> {
    let grid = parse_input(input);
    let rows = grid.len();
    if rows == 0 {
        return Ok(0);
    }
    let cols = grid[0].len();

    // Target word as bytes for fast comparison
    const TARGET: &[u8] = b"XMAS";

    // All 8 directions: (row_delta, col_delta)
    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    let mut count = 0;

    // Check each starting position in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Check all 8 directions from this position
            for &(dr, dc) in &DIRECTIONS {
                if check_word_at_position(&grid, row, col, dr, dc, TARGET) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

/// Checks if the target word appears at a specific position in a specific direction.
///
/// Uses bounds checking and early termination for optimal performance.
///
/// # Parameters
/// * `grid` - The 2D grid of characters as bytes
/// * `start_row` - Starting row position
/// * `start_col` - Starting column position
/// * `dr` - Row direction delta (-1, 0, or 1)
/// * `dc` - Column direction delta (-1, 0, or 1)
/// * `target` - Target word as byte slice
///
/// # Returns
/// `true` if the target word is found starting at the given position in the given direction
fn check_word_at_position(
    grid: &[Vec<u8>],
    start_row: usize,
    start_col: usize,
    dr: i32,
    dc: i32,
    target: &[u8],
) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    // Pre-check if we can fit the entire word in this direction
    let end_row = start_row as i32 + dr * (target.len() as i32 - 1);
    let end_col = start_col as i32 + dc * (target.len() as i32 - 1);

    if end_row < 0 || end_row >= rows as i32 || end_col < 0 || end_col >= cols as i32 {
        return false;
    }

    // Check each character in the target word
    for (i, &target_char) in target.iter().enumerate() {
        let current_row = (start_row as i32 + dr * i as i32) as usize;
        let current_col = (start_col as i32 + dc * i as i32) as usize;

        if grid[current_row][current_col] != target_char {
            return false;
        }
    }

    true
}
