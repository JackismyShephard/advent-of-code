use anyhow::Result;
use day01::{solve_part1, solve_part2, EXAMPLE_INPUT};

fn main() -> Result<()> {
    // Test with example input
    let result1 = solve_part1(EXAMPLE_INPUT)?;
    println!("Part 1 example result: {result1}");

    let result2 = solve_part2(EXAMPLE_INPUT)?;
    println!("Part 2 example result: {result2}");

    // Try to read actual input file if it exists
    if let Ok(input) = std::fs::read_to_string("day01/input.txt") {
        let result1 = solve_part1(&input)?;
        println!("Part 1 result: {result1}");

        let result2 = solve_part2(&input)?;
        println!("Part 2 result: {result2}");
    } else {
        println!("No input.txt found - create day01/input.txt with your puzzle input");
    }

    Ok(())
}
