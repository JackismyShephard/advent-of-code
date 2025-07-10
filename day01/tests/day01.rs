use day01::{parse_input, solve_part1, solve_part2, solve_part2_naive, EXAMPLE_INPUT};
use rstest::rstest;

// ===== PARSE INPUT TESTS =====

#[test]
fn test_parse_input_example() {
    let (left, right) = parse_input(EXAMPLE_INPUT).unwrap();
    assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
}

#[rstest]
#[case("", vec![], vec![], "empty input")] // Empty input
#[case("1", vec![], vec![], "single number")] // Single number, no pair
#[case("1  2", vec![1], vec![2], "extra spaces")] // Extra spaces are handled
#[case("1 2\n3", vec![1], vec![2], "incomplete pair")] // Incomplete pair is skipped
fn test_parse_input_edge_cases(
    #[case] input: &str,
    #[case] expected_left: Vec<i32>,
    #[case] expected_right: Vec<i32>,
    #[case] description: &str,
) {
    let (left, right) = parse_input(input).unwrap();
    assert_eq!(left, expected_left, "Left mismatch for {description}");
    assert_eq!(right, expected_right, "Right mismatch for {description}");
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, 11)] // Part 1 with example input
#[case(solve_part2, 31)] // Part 2 with example input
#[case(solve_part2_naive, 31)] // Part 2 naive with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let result = solve_fn(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("1 2\n3 4", solve_part1, 2)] // Simple case: sorted [1,3] and [2,4] -> |1-2| + |3-4| = 1 + 1 = 2
#[case("1 2\n3 4", solve_part2, 0)] // No similarity (no common numbers)
#[case("1 2\n3 4", solve_part2_naive, 0)] // Same as above
#[case("", solve_part1, 0)] // Empty input edge case
#[case("", solve_part2, 0)] // Empty input edge case
#[case("", solve_part2_naive, 0)] // Empty input edge case
fn test_solve_functions_edge_cases(
    #[case] input: &str,
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, 1603498)] // Part 1 with real input
#[case(solve_part2, 25574739)] // Part 2 with real input
#[case(solve_part2_naive, 25574739)] // Part 2 naive with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<i32>,
    #[case] expected: i32,
) {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
