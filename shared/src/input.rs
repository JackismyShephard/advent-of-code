//! Shared input reading and parsing utilities for Advent of Code challenges.
//!
//! This module provides common functionality for reading puzzle input files
//! and parsing them into commonly used formats across different days.

use anyhow::Result;
use std::fs;

/// Reads the puzzle input file for a specific Advent of Code day.
///
/// Constructs the standard input file path and reads the entire file contents
/// into memory as a UTF-8 string. Follows the naming convention
/// `dayXX/input.txt`.
///
/// # Parameters
/// * `day` - The day number (1-25) for which to read the input file
///
/// # Returns
/// Complete file contents as a UTF-8 string with original formatting preserved
///
/// # Errors
///
/// Returns an error if:
/// - The input file doesn't exist at the expected path
/// - File cannot be read due to permissions or I/O errors
/// - File contains invalid UTF-8 sequences
///
/// # Examples
///
/// ```
/// # use shared::input::read_input;
/// # use std::fs;
/// # use std::io::Write;
/// # // Create a test input file
/// # fs::create_dir_all("day01").unwrap();
/// # let mut file = fs::File::create("day01/input.txt").unwrap();
/// # writeln!(file, "123\n456").unwrap();
/// let input = read_input(1)?;
/// assert_eq!(input, "123\n456\n");
/// # fs::remove_file("day01/input.txt").unwrap();
/// # fs::remove_dir("day01").unwrap();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn read_input(day: u8) -> Result<String> {
    let filename = format!("day{day:02}/input.txt");
    Ok(fs::read_to_string(filename)?)
}
