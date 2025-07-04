use day03::{solve_part1, EXAMPLE_INPUT};

fn main() {
    println!("=== Day 3: Mull It Over ===");
    println!();

    // Test with example input
    println!("=== Example Input Results ===");
    let example_result = solve_part1(EXAMPLE_INPUT);
    println!("Part 1 example result: {example_result}");
    println!();

    // Run on real input
    println!("=== Real Input Results ===");
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let part1_result = solve_part1(&input);
    println!("Part 1 result: {part1_result}");
}
