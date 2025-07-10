use day04::*;
use rstest::rstest;

// ===== PARSE INPUT TESTS =====

#[test]
fn test_parse_input_example() {
    let grid = parse_input(EXAMPLE_INPUT);
    assert_eq!(grid.len(), 10); // 10 rows
    assert_eq!(grid[0].len(), 10); // 10 columns
    assert_eq!(
        grid[0],
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
    ); // First row
    assert_eq!(
        grid[9],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
    ); // Last row
}

#[rstest]
#[case("", vec![], "empty input")] // Empty input
#[case("A", vec![vec!['A']], "single character")] // Single character
#[case("ABC", vec![vec!['A', 'B', 'C']], "single line")] // Single line
#[case("A\nB", vec![vec!['A'], vec!['B']], "single column")] // Single column
#[case("AB\nCD\nEF", vec![vec!['A', 'B'], vec!['C', 'D'], vec!['E', 'F']], "multiple lines")] // Multiple lines
#[case("ABC\nDE", vec![vec!['A', 'B', 'C'], vec!['D', 'E']], "different line lengths")] // Different line lengths
fn test_parse_input_edge_cases(
    #[case] input: &str,
    #[case] expected: Vec<Vec<char>>,
    #[case] description: &str,
) {
    let grid = parse_input(input);
    assert_eq!(grid, expected, "Failed for {description}");
}

// ===== CORE FUNCTION TESTS =====

#[rstest]
#[case("XMAS\nABCD", 0, 0, 0, 1, true)] // Horizontal right: "XMAS" from (0,0)
#[case("XMAS\nABCD", 0, 0, 0, -1, false)] // Horizontal left: no "XMAS" from (0,0)
#[case("SAMX\nABCD", 0, 3, 0, -1, true)] // Horizontal left: "SAMX" backwards is "XMAS"
#[case("X\nM\nA\nS", 0, 0, 1, 0, true)] // Vertical down: "XMAS" from (0,0)
#[case("X\nM\nA\nS", 0, 0, -1, 0, false)] // Vertical up: no "XMAS" from (0,0)
#[case("S\nA\nM\nX", 3, 0, -1, 0, true)] // Vertical up: "SAMX" upwards is "XMAS"
#[case("X...\n.M..\n..A.\n...S", 0, 0, 1, 1, true)] // Diagonal down-right: "XMAS" from (0,0)
#[case("S...\n.A..\n..M.\n...X", 3, 3, -1, -1, true)] // Diagonal up-left: "SAMX" up-left is "XMAS"
fn test_check_direction_patterns(
    #[case] grid_input: &str,
    #[case] row: usize,
    #[case] col: usize,
    #[case] delta_row: isize,
    #[case] delta_col: isize,
    #[case] expected: bool,
) {
    let grid = parse_input(grid_input);
    assert_eq!(
        check_direction(&grid, row, col, delta_row, delta_col),
        expected
    );
}

#[rstest]
#[case("XM\nAS", 0, 0, 0, 1, false)] // Horizontal: can't fit "XMAS" in 2x2 grid
#[case("XM\nAS", 0, 0, 1, 0, false)] // Vertical: can't fit "XMAS" in 2x2 grid
#[case("XM\nAS", 0, 0, 1, 1, false)] // Diagonal: can't fit "XMAS" in 2x2 grid
fn test_check_direction_bounds(
    #[case] grid_input: &str,
    #[case] row: usize,
    #[case] col: usize,
    #[case] delta_row: isize,
    #[case] delta_col: isize,
    #[case] expected: bool,
) {
    let grid = parse_input(grid_input);
    assert_eq!(
        check_direction(&grid, row, col, delta_row, delta_col),
        expected
    );
}

#[test]
fn test_count_xmas_at_position() {
    // Test with a grid that has clear XMAS patterns
    let grid = parse_input("XMAS\nM...\nA...\nS...");

    // Position (0,0) has 'X' - should find XMAS going right and down
    let count = count_xmas_at_position(&grid, 0, 0);
    assert_eq!(count, 2); // Right: XMAS, Down: XMAS

    // Position (0,1) has 'M' - should find no XMAS starting from M
    let count = count_xmas_at_position(&grid, 0, 1);
    assert_eq!(count, 0); // No XMAS patterns start with M
}

#[rstest]
#[case("M.S\n.A.\nM.S", 1, 1, true)] // Center A with X-MAS pattern
#[case("S.M\n.A.\nS.M", 1, 1, true)] // SAM variant
#[case("M.M\n.A.\nS.S", 1, 1, true)] // Both diagonals MAS
#[case("S.S\n.A.\nM.M", 1, 1, true)] // Both diagonals SAM
#[case("M.S\n.X.\nM.S", 1, 1, false)] // No 'A' at center
#[case("M.S\n.A.\nX.Y", 1, 1, false)] // Wrong characters on diagonals
#[case("AB\nCD", 0, 0, false)] // Too small grid
#[case("AB\nCD", 1, 1, false)] // Out of bounds for pattern
fn test_is_xmas_pattern_variants(
    #[case] grid_input: &str,
    #[case] row: usize,
    #[case] col: usize,
    #[case] expected: bool,
) {
    let grid = parse_input(grid_input);
    assert_eq!(is_xmas_pattern(&grid, row, col), expected);
}

#[test]
fn test_is_xmas_pattern_different_positions() {
    let input = ".....\n.M.S.\n..A..\n.M.S.\n.....";
    let grid = parse_input(input);
    assert!(is_xmas_pattern(&grid, 2, 2)); // X-MAS pattern at (2,2) instead of (1,1)
    assert!(!is_xmas_pattern(&grid, 1, 1)); // No pattern at (1,1)
}

// ===== SOLVE FUNCTION TESTS =====

#[rstest]
#[case(solve_part1, EXAMPLE_INPUT, 18)] // Part 1 with example input
#[case(solve_part2, EXAMPLE_INPUT, 9)] // Part 2 with example input
fn test_solve_functions_example(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] input: &str,
    #[case] expected: usize,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, "XMAS\nMASX", 1)] // Simple: Row 0 has "XMAS" going right
#[case(solve_part1, "XMAS", 1)] // Single line: "XMAS" going right
#[case(solve_part1, "X\nM\nA\nS", 1)] // Vertical: "XMAS" going down
#[case(solve_part1, "", 0)] // Empty input
#[case(solve_part1, "ABCD\nEFGH", 0)] // No matches
#[case(solve_part2, "M.S\n.A.\nM.S", 1)] // Single X-MAS pattern
#[case(solve_part2, "ABC\nDEF\nGHI", 0)] // No patterns
#[case(solve_part2, "", 0)] // Empty input
#[case(solve_part2, "AB\nCD", 0)] // Grid too small for X-MAS pattern
#[case(solve_part2, "M.S.M.S\n.A...A.\nM.S.M.S", 2)] // Multiple X-MAS patterns
fn test_solve_functions_edge_cases(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] input: &str,
    #[case] expected: usize,
) {
    let result = solve_fn(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(solve_part1, 2447)] // Part 1 with real input
#[case(solve_part2, 1868)] // Part 2 with real input
fn test_solve_functions_real_input(
    #[case] solve_fn: fn(&str) -> anyhow::Result<usize>,
    #[case] expected: usize,
) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = solve_fn(&input).unwrap();
    assert_eq!(result, expected);
}
