use day04::*;

#[test]
fn test_parse_input() {
    let input = "ABC\nDEF";
    let grid = parse_input(input);
    assert_eq!(grid, vec![vec!['A', 'B', 'C'], vec!['D', 'E', 'F']]);
}

#[test]
fn test_check_direction_horizontal() {
    let grid = parse_input("XMAS\nABCD");

    // Test horizontal right - should find "XMAS" from (0,0)
    assert!(check_direction(&grid, 0, 0, 0, 1));

    // Test horizontal left - no "XMAS" going left from position (0,0)
    assert!(!check_direction(&grid, 0, 0, 0, -1));

    // Test horizontal left from end position - "SAMX" backwards is "XMAS"
    let grid2 = parse_input("SAMX\nABCD");
    assert!(check_direction(&grid2, 0, 3, 0, -1));
}

#[test]
fn test_check_direction_vertical() {
    let grid = parse_input("X\nM\nA\nS");

    // Test vertical down - should find "XMAS" from (0,0)
    assert!(check_direction(&grid, 0, 0, 1, 0));

    // Test vertical up - no "XMAS" going up from position (0,0)
    assert!(!check_direction(&grid, 0, 0, -1, 0));

    // Test vertical up from bottom - "SAMX" upwards is "XMAS"
    let grid2 = parse_input("S\nA\nM\nX");
    assert!(check_direction(&grid2, 3, 0, -1, 0));
}

#[test]
fn test_check_direction_diagonal() {
    let grid = parse_input("X...\n.M..\n..A.\n...S");

    // Test diagonal down-right - should find "XMAS" from (0,0)
    assert!(check_direction(&grid, 0, 0, 1, 1));

    // Test diagonal up-left from bottom-right - "SAMX" up-left is "XMAS"
    let grid2 = parse_input("S...\n.A..\n..M.\n...X");
    assert!(check_direction(&grid2, 3, 3, -1, -1));
}

#[test]
fn test_check_direction_bounds() {
    let grid = parse_input("XM\nAS");

    // Test that bounds checking works - can't fit "XMAS" in a 2x2 grid
    assert!(!check_direction(&grid, 0, 0, 0, 1)); // would need 4 characters horizontally
    assert!(!check_direction(&grid, 0, 0, 1, 0)); // would need 4 characters vertically
    assert!(!check_direction(&grid, 0, 0, 1, 1)); // would need 4 characters diagonally
}

#[test]
fn test_count_xmas_at_position() {
    // Test a specific position where we can count exactly
    let grid = parse_input("SAMX\nAXAS\nMASX\nXSAM");

    // Position (1,1) has 'X' - let's check what directions work
    let count = count_xmas_at_position(&grid, 1, 1);
    // From (1,1) with 'X': right gives "XMAS" if we have X-M-A-S
    // Need to manually verify this specific case
    assert_eq!(count, 0); // This particular grid doesn't have XMAS from (1,1)
}

#[test]
fn test_solve_part1_example() {
    let result = solve_part1(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 18); // Given in problem statement
}

#[test]
fn test_solve_part1_simple() {
    let simple_input = "XMAS\nMASX";
    let result = solve_part1(simple_input).unwrap();
    // Row 0: "XMAS" going right from (0,0) = 1 match
    // Row 1: No additional XMAS patterns found
    assert_eq!(result, 1);
}

#[test]
fn test_solve_part1_single_xmas() {
    let input = "XMAS";
    let result = solve_part1(input).unwrap();
    // Only one "XMAS" going right from (0,0)
    assert_eq!(result, 1);
}

#[test]
fn test_solve_part1_vertical() {
    let input = "X\nM\nA\nS";
    let result = solve_part1(input).unwrap();
    // Only one "XMAS" going down from (0,0)
    assert_eq!(result, 1);
}

#[test]
fn test_solve_part1_empty() {
    let result = solve_part1("").unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_solve_part1_no_matches() {
    let input = "ABCD\nEFGH";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_solve_part1_real_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = solve_part1(&input).unwrap();

    assert_eq!(result, 2447);
}
