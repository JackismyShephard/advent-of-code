//! Day 3: Mull It Over
//!
//! Solution for Advent of Code 2024 Day 3.
//!
//! Part 1: Parse corrupted memory to find valid mul(X,Y) instructions
//! and sum their results. Valid instructions have the exact format
//! mul(X,Y) where X and Y are 1-3 digit numbers.

use anyhow::Result;
use regex::Regex;
use std::sync::LazyLock;

/// Compiled regex pattern for matching mul(X,Y) instructions.
///
/// Uses LazyLock to compile the regex only once on first use.
static MUL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern"));

/// Example input used for testing and documentation.
pub const EXAMPLE_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

/// Extracts all valid mul(X,Y) instructions from corrupted memory.
///
/// Uses regex pattern matching to find instructions with the exact format
/// mul(X,Y) where X and Y are 1-3 digit numbers. Invalid formats like
/// mul(4*, mul[3,7], or mul ( 2 , 4 ) are ignored.
///
/// # Parameters
/// * `memory` - String containing corrupted memory with mixed valid/invalid instructions
///
/// # Returns
/// Vector of (X, Y) tuples representing the operands of valid mul instructions
///
/// # Errors
///
/// Returns `Err` if any captured number cannot be parsed as a u32.
///
/// # Examples
///
/// ```
/// # use day03::extract_mul_instructions;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// let instructions = extract_mul_instructions(memory).unwrap();
/// assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
/// ```
pub fn extract_mul_instructions(memory: &str) -> Result<Vec<(u32, u32)>> {
    MUL_PATTERN
        .captures_iter(memory)
        .map(|captures| {
            let x = captures[1].parse()?;
            let y = captures[2].parse()?;
            Ok((x, y))
        })
        .collect()
}

/// Solves Part 1: Sums the results of all valid multiplication instructions.
///
/// Scans corrupted memory for valid mul(X,Y) instructions, multiplies the
/// operands, and returns the sum of all multiplication results.
///
/// # Parameters
/// * `input` - String containing corrupted memory to parse
///
/// # Returns
/// Sum of all multiplication results as a u32
///
/// # Errors
///
/// Returns `Err` if instruction parsing fails (malformed numbers).
///
/// # Examples
///
/// ```
/// # use day03::solve_part1;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// assert_eq!(solve_part1(memory).unwrap(), 161); // 2*4 + 5*5 + 11*8 + 8*5 = 161
/// ```
pub fn solve_part1(input: &str) -> Result<u32> {
    let instructions = extract_mul_instructions(input)?;
    Ok(instructions.iter().map(|(x, y)| x * y).sum())
}
