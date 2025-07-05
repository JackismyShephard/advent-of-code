use day03::{solve_part1, solve_part2, EXAMPLE_INPUT, EXAMPLE_INPUT_PART2};

fn main() -> anyhow::Result<()> {
    println!("=== Day 3: Mull It Over ===");
    println!();

    // Test with example input
    println!("=== Example Input Results ===");
    let example_result_part1 = solve_part1(EXAMPLE_INPUT)?;
    println!("Part 1 example result: {example_result_part1}");

    let example_result_part2 = solve_part2(EXAMPLE_INPUT_PART2)?;
    println!("Part 2 example result: {example_result_part2}");
    println!();

    // Run on real input
    println!("=== Real Input Results ===");
    let input = std::fs::read_to_string("input.txt")?;

    let part1_result = solve_part1(&input)?;
    println!("Part 1 result: {part1_result}");

    let part2_result = solve_part2(&input)?;
    println!("Part 2 result: {part2_result}");

    Ok(())
}
