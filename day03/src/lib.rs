//! Day 3: Mull It Over
//!
//! Solution for Advent of Code 2024 Day 3.
//!
//! Part 1: Parse corrupted memory to find valid mul(X,Y) instructions
//! and sum their results. Valid instructions have the exact format
//! mul(X,Y) where X and Y are 1-3 digit numbers.

use regex::Regex;

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
/// # Examples
///
/// ```
/// # use day03::extract_mul_instructions;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// let instructions = extract_mul_instructions(memory);
/// assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
/// ```
pub fn extract_mul_instructions(memory: &str) -> Vec<(u32, u32)> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    pattern
        .captures_iter(memory)
        .map(|captures| {
            let x = captures[1].parse::<u32>().unwrap();
            let y = captures[2].parse::<u32>().unwrap();
            (x, y)
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
/// This function does not return errors as regex parsing is infallible
/// for this specific pattern.
///
/// # Examples
///
/// ```
/// # use day03::solve_part1;
/// let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// assert_eq!(solve_part1(memory), 161); // 2*4 + 5*5 + 11*8 + 8*5 = 161
/// ```
pub fn solve_part1(input: &str) -> u32 {
    extract_mul_instructions(input)
        .iter()
        .map(|(x, y)| x * y)
        .sum()
}
