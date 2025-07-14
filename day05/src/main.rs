use anyhow::Result;
use day05::{solve_part1, EXAMPLE_INPUT};

fn main() -> Result<()> {
    println!("=== Day 5: Print Queue ===");
    println!();

    // Test with example input
    println!("=== Example Input Results ===");
    let result1 = solve_part1(EXAMPLE_INPUT)?;
    println!("Part 1 example result: {result1}");

    // Try to read actual input file if it exists
    if let Ok(input) = std::fs::read_to_string("day05/input.txt") {
        println!("\n=== Real Input Results ===");
        let result1 = solve_part1(&input)?;
        println!("Part 1 result: {result1}");
    } else {
        println!("No input.txt found - create day05/input.txt with your puzzle input");
    }

    Ok(())
}
