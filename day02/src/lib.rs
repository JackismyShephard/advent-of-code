//! Day 2: Red-Nosed Reports
//!
//! Solution for Advent of Code 2024 Day 2.
//!
//! Part 1: Analyze reactor safety reports to determine which are safe.
//! A report is safe if levels are all increasing or all decreasing,
//! and adjacent levels differ by 1-3.

use anyhow::Result;
use shared::input::parse_lines;

/// Example input used for testing and documentation.
pub const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

/// Parses the input string into a vector of reports, where each report is a vector of levels.
///
/// Each line contains space-separated integers representing reactor levels.
///
/// # Parameters
/// * `input` - Multi-line string with reactor level reports (one report per line, space-separated integers)
///
/// # Returns
/// Vector of reports, where each report is a Vec<i32> of levels
///
/// # Errors
///
/// Returns `Err` if any value cannot be parsed as an `i32`.
///
/// # Examples
///
/// ```
/// # use day02::parse_input;
/// let input = "1 2 3\n4 5 6";
/// let reports = parse_input(input).unwrap();
/// assert_eq!(reports, vec![vec![1, 2, 3], vec![4, 5, 6]]);
/// ```
pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    let lines = parse_lines(input);
    let mut reports = Vec::new();

    for line in lines {
        let levels = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        reports.push(levels);
    }

    Ok(reports)
}

/// Checks if a report is safe according to reactor safety rules.
///
/// A report is safe if:
/// 1. All levels are either increasing or decreasing
/// 2. Adjacent levels differ by at least 1 and at most 3
///
/// # Parameters
/// * `report` - Vector of reactor levels to analyze for safety
///
/// # Returns
/// True if the report meets all safety criteria, false otherwise
///
/// # Examples
///
/// ```
/// # use day02::is_safe;
/// assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);  // Decreasing by 1-2
/// assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false); // Jump of 5
/// assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false); // No change (4->4)
/// ```
pub fn is_safe(report: &[i32]) -> bool {
    let mut direction = None;

    report.windows(2).all(|window| {
        let diff = window[1] - window[0];

        // Check if difference is within valid range (1-3)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check/establish monotonicity
        let is_increasing = diff > 0;
        match direction {
            None => {
                direction = Some(is_increasing);
                true
            }
            Some(dir) => dir == is_increasing,
        }
    })
}

/// Solves Part 1: Counts how many reports are safe.
///
/// Analyzes each reactor report to determine if it meets safety criteria:
/// all levels increasing/decreasing with adjacent differences of 1-3.
///
/// # Parameters
/// * `input` - Multi-line string containing reactor level reports
///
/// # Returns
/// Number of safe reports as an integer
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day02::solve_part1;
/// let input = "7 6 4 2 1\n1 3 6 7 9";
/// assert_eq!(solve_part1(input).unwrap(), 2); // Both reports are safe
/// ```
pub fn solve_part1(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    Ok(safe_count)
}

/// Pure functional implementation of safety checking for performance comparison.
///
/// Uses multiple iterator passes to separate concerns: collect differences,
/// then check range constraints and monotonicity independently.
///
/// # Parameters
/// * `report` - Vector of reactor levels to analyze for safety
///
/// # Returns
/// True if the report meets all safety criteria, false otherwise
///
/// # Examples
///
/// ```
/// # use day02::is_safe_functional;
/// assert!(is_safe_functional(&[7, 6, 4, 2, 1]));  // Decreasing by 1-2
/// assert!(!is_safe_functional(&[1, 2, 7, 8, 9])); // Jump of 5
/// ```
pub fn is_safe_functional(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    // Check all differences are in valid range (1-3)
    let valid_range = diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3);

    // Check all same direction (all positive or all negative)
    let monotonic = diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0);

    valid_range && monotonic
}

/// Solves part 1 using the functional approach for safety analysis.
///
/// Provides an alternative implementation using the functional `is_safe_functional`
/// method for comparison and verification purposes. Should produce identical
/// results to `solve_part1`.
///
/// # Parameters
/// * `input` - Multi-line string containing reactor safety reports
///
/// # Returns
/// Number of safe reports according to functional analysis approach
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day02::{solve_part1_functional, EXAMPLE_INPUT};
/// let result = solve_part1_functional(EXAMPLE_INPUT).unwrap();
/// assert_eq!(result, 2);
/// ```
pub fn solve_part1_functional(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_functional(report))
        .count();
    Ok(safe_count)
}

/// Checks if a report is safe with the Problem Dampener active.
///
/// The Problem Dampener allows removing exactly one level from an unsafe report
/// to make it safe. A report is considered safe if it's either already safe,
/// or becomes safe after removing any single level.
///
/// # Parameters
/// * `report` - Vector of reactor levels to analyze with dampening capability
///
/// # Returns
/// True if the report is safe or can be made safe by removing one level
///
/// # Examples
///
/// ```
/// # use day02::is_safe_with_dampener;
/// assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1])); // Already safe
/// assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5])); // Safe by removing 3
/// assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1])); // Safe by removing one 4
/// assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9])); // Unsafe regardless
/// ```
pub fn is_safe_with_dampener(report: &[i32]) -> bool {
    // First check if already safe
    if is_safe(report) {
        return true;
    }

    // Try removing each level one by one
    for i in 0..report.len() {
        // Create a new vector without the element at index i
        let modified: Vec<i32> = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .copied()
            .collect();

        if is_safe(&modified) {
            return true;
        }
    }

    false
}

/// Solves Part 2: Counts how many reports are safe with the Problem Dampener.
///
/// Analyzes each reactor report to determine if it meets safety criteria
/// either directly or after removing exactly one level. The Problem Dampener
/// allows the reactor safety systems to tolerate a single bad level.
///
/// # Parameters
/// * `input` - Multi-line string containing reactor level reports
///
/// # Returns
/// Number of safe reports (including those made safe by dampening) as an integer
///
/// # Errors
///
/// Returns `Err` if input parsing fails.
///
/// # Examples
///
/// ```
/// # use day02::solve_part2;
/// let input = "7 6 4 2 1\n1 3 2 4 5\n8 6 4 4 1";
/// assert_eq!(solve_part2(input).unwrap(), 3); // All can be made safe
/// ```
pub fn solve_part2(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    Ok(safe_count)
}
