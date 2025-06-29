use anyhow::Result;
use shared::parse_lines;

fn solve_part1(input: &str) -> Result<i32> {
    let lines = parse_lines(input);

    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    // Parse the two columns of numbers
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_nums.push(parts[0].parse::<i32>()?);
            right_nums.push(parts[1].parse::<i32>()?);
        }
    }

    // Sort both lists
    left_nums.sort();
    right_nums.sort();

    // Calculate total distance
    let total_distance: i32 = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(total_distance)
}

fn main() -> Result<()> {
    // Test with example input
    let example_input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let result = solve_part1(example_input)?;
    println!("Example result: {result} ✅");

    // Try to read actual input file if it exists
    if let Ok(input) = std::fs::read_to_string("day01/input.txt") {
        let result = solve_part1(&input)?;
        println!("Part 1 result: {result}");
    } else {
        println!("No input.txt found - create day01/input.txt with your puzzle input");
    }

    Ok(())
}
