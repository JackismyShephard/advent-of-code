use anyhow::Result;
use day04::{solve_part1, EXAMPLE_INPUT};

fn main() -> Result<()> {
    // Test with example input
    let result1 = solve_part1(EXAMPLE_INPUT)?;
    println!("Part 1 example result: {result1}");

    // Try to read actual input file if it exists
    if let Ok(input) = std::fs::read_to_string("day04/input.txt") {
        let result1 = solve_part1(&input)?;
        println!("Part 1 result: {result1}");
    } else {
        println!("No input.txt found - create day04/input.txt with your puzzle input");
    }

    Ok(())
}
