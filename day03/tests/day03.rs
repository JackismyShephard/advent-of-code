use day03::{
    extract_enabled_mul_instructions, extract_mul_instructions, solve_part1, solve_part2,
    EXAMPLE_INPUT, EXAMPLE_INPUT_PART2,
};
use rstest::rstest;

// ===== CORE FUNCTION TESTS =====

#[test]
fn test_extract_mul_instructions_example() {
    let instructions = extract_mul_instructions(EXAMPLE_INPUT).unwrap();
    assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
}

#[rstest]
#[case("mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 ) mul[3,7] mul(123,456)", vec![(123, 456)])] // Invalid formats ignored
#[case("", vec![])] // Empty input
#[case("no mul instructions here", vec![])] // No valid instructions
#[case("mul(1,2) mul(12,34) mul(123,456) mul(1234,5) mul(1,2345)", vec![(1, 2), (12, 34), (123, 456)])] // 1-3 digit boundary
fn test_extract_mul_instructions_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let instructions = extract_mul_instructions(input).unwrap();
    assert_eq!(instructions, expected);
}

#[test]
fn test_extract_enabled_mul_instructions_examples() {
    let instructions = extract_enabled_mul_instructions(EXAMPLE_INPUT_PART2).unwrap();
    assert_eq!(instructions, vec![(2, 4), (8, 5)]);
}

#[rstest]
#[case("mul(3,4)don't()mul(5,6)", vec![(3, 4)])] // Instructions enabled by default
#[case("mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)", vec![(1, 1), (3, 3), (5, 5)])] // Multiple state changes
#[case("mul(1,2)mul(3,4)mul(5,6)", vec![(1, 2), (3, 4), (5, 6)])] // No state changes
#[case("don't()mul(1,2)mul(3,4)mul(5,6)", vec![])] // All disabled
#[case("", vec![])] // Empty input
fn test_extract_enabled_mul_instructions_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<(u32, u32)>,
) {
    let instructions = extract_enabled_mul_instructions(input).unwrap();
    assert_eq!(instructions, expected);
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, EXAMPLE_INPUT, 161)] // Part 1 with example input
#[case(solve_part2, EXAMPLE_INPUT_PART2, 48)] // Part 2 with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] input: &str,
    #[case] expected: u32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, "mul(2,3)", 6)] // Simple multiplication
#[case(solve_part1, "mul(10,10)", 100)] // Two-digit numbers
#[case(solve_part1, "mul(1,1)mul(2,2)mul(3,3)", 14)] // Multiple instructions: 1 + 4 + 9 = 14
#[case(solve_part1, "no valid instructions", 0)] // No valid instructions
#[case(solve_part1, "mul(4* mul[3,7] mul ( 2 , 4 )", 0)] // Invalid format instructions
#[case(solve_part2, "mul(2,3)", 6)] // Simple enabled case
#[case(solve_part2, "don't()mul(2,3)", 0)] // Simple disabled case
#[case(solve_part2, "don't()mul(2,3)do()mul(4,5)", 20)] // Re-enabled case
#[case(
    solve_part2,
    "mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)",
    35
)] // Complex state changes: 1*1 + 3*3 + 5*5 = 35
fn test_solve_functions_edge_cases(
    #[case] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] input: &str,
    #[case] expected: u32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, 167650499)] // Part 1 with real input
#[case(solve_part2, 95846796)] // Part 2 with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<u32>,
    #[case] expected: u32,
) {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
