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

pub const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

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

pub fn solve_part2_naive(input: &str) -> Result<i32> {
    let (left_nums, right_nums) = parse_input(input)?;

    let mut similarity_score = 0;

    // O(n²) approach: for each number in left list, count occurrences in right list
    for &left_num in &left_nums {
        let count = right_nums
            .iter()
            .filter(|&&right_num| right_num == left_num)
            .count() as i32;
        similarity_score += left_num * count;
    }

    Ok(similarity_score)
}
