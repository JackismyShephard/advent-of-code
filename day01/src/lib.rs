//! Day 1: Historian Hysteria
//!
//! Solution for Advent of Code 2024 Day 1.
//!
//! Part 1: Calculate total distance between two sorted lists by pairing up
//! the smallest numbers and summing the absolute differences.
//!
//! Part 2: Calculate similarity score by multiplying each number in the left
//! list by how many times it appears in the right list, then summing.

use anyhow::Result;
use rustc_hash::FxHashMap;
use shared::parse_lines;

/// Example input from the problem statement used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

/// Parses the input string into two separate lists of integers (left and right columns).
///
/// Takes input with one pair of integers per line, separated by whitespace,
/// and separates them into left and right column vectors.
///
/// # Parameters
/// * `input` - Multi-line string with integer pairs (one pair per line, whitespace-separated)
///
/// # Returns
/// Tuple of (left_column_numbers, right_column_numbers) as Vec<i32>
///
/// # Errors
///
/// Returns `Err` if any value cannot be parsed as an `i32`.
///
/// # Examples
///
/// ```
/// # use day01::parse_input;
/// let input = "1 2\n3 4";
/// let (left, right) = parse_input(input).unwrap();
/// assert_eq!(left, vec![1, 3]);
/// assert_eq!(right, vec![2, 4]);
/// ```
pub fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let lines = parse_lines(input);

    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();

    // Parse the two columns of numbers
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_nums.push(parts[0].parse()?);
            right_nums.push(parts[1].parse()?);
        }
    }

    Ok((left_nums, right_nums))
}

/// Solves Part 1: Calculates the total distance between the sorted left and right lists.
///
/// The function sorts both lists independently and then sums the absolute differences
/// of corresponding elements when paired by position.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs (whitespace-separated)
///
/// # Returns
/// Total distance as the sum of absolute differences between sorted pairs
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day01::solve_part1;
/// let input = "1 3\n2 5";
/// assert_eq!(solve_part1(input).unwrap(), 5); // |1-3| + |2-5| = 2 + 3 = 5
/// ```
pub fn solve_part1(input: &str) -> Result<i32> {
    let (mut left_nums, mut right_nums) = parse_input(input)?;

    // Sort both lists
    left_nums.sort();
    right_nums.sort();

    // Calculate total distance
    let total_distance = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(total_distance)
}

/// Solves Part 2: Calculates a similarity score based on frequency matching.
///
/// Multiplies each number in the left list by how many times it appears in the right list.
/// Uses hash maps for efficient frequency counting and handles duplicate values optimally.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs (whitespace-separated)
///
/// # Returns
/// Similarity score as the sum of (left_number × left_frequency × right_frequency)
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day01::solve_part2;
/// let input = "3 3\n4 3\n2 3";
/// // 3 appears 3 times in right list: 3*3 = 9
/// // 4 appears 0 times in right list: 4*0 = 0
/// // 2 appears 0 times in right list: 2*0 = 0
/// assert_eq!(solve_part2(input).unwrap(), 9); // 9 + 0 + 0 = 9
/// ```
pub fn solve_part2(input: &str) -> Result<i32> {
    let (left_nums, right_nums) = parse_input(input)?;

    // Build frequency map for right list
    let mut right_counts = FxHashMap::default();
    for &num in &right_nums {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Build frequency map for left list (optimization for duplicates)
    let mut left_counts = FxHashMap::default();
    for &num in &left_nums {
        *left_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    let mut similarity_score = 0;
    for (left_num, left_freq) in &left_counts {
        let right_freq = right_counts.get(left_num).unwrap_or(&0);
        similarity_score += left_num * left_freq * right_freq;
    }

    Ok(similarity_score)
}

/// Naive O(n²) implementation of Part 2 for performance comparison.
///
/// Uses nested iteration to count occurrences without hash map optimization.
///
/// # Parameters
/// * `input` - Multi-line string containing integer pairs (whitespace-separated)
///
/// # Returns
/// Similarity score calculated using the naive O(n²) algorithm
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day01::solve_part2_naive;
/// let input = "3 3\n4 3\n2 3";
/// assert_eq!(solve_part2_naive(input).unwrap(), 9); // Same result as optimized version
/// ```
pub fn solve_part2_naive(input: &str) -> Result<i32> {
    let (left_nums, right_nums) = parse_input(input)?;

    let mut similarity_score = 0;

    // O(n²) approach: for each number in left list, count occurrences in right list
    for left_num in &left_nums {
        let count = right_nums
            .iter()
            .filter(|&right_num| right_num == left_num)
            .count() as i32;
        similarity_score += left_num * count;
    }

    Ok(similarity_score)
}
