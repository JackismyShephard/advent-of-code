use day04::{parse_input, solve_part1, EXAMPLE_INPUT};

#[test]
fn test_part1_example() {
    let result = solve_part1(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 18);
}

#[test]
fn test_parse_input() {
    let grid = parse_input(EXAMPLE_INPUT);
    assert_eq!(grid.len(), 10);
    assert_eq!(grid[0], b"MMMSXXMASM");
    assert_eq!(grid[1], b"MSAMXMSMSA");
}

#[test]
fn test_simple_cases() {
    // Test simple horizontal cases
    let input = "XMAS";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);

    // Test backwards
    let input = "SAMX";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);

    // Test vertical
    let input = "X\nM\nA\nS";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);

    // Test vertical backwards
    let input = "S\nA\nM\nX";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_diagonal_cases() {
    // Test diagonal down-right
    let input = "X...\n.M..\n..A.\n...S";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);

    // Test diagonal up-left (same as down-right backwards)
    let input = "S...\n.A..\n..M.\n...X";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_part1_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part1(&input).unwrap();
    // We don't know the expected result yet, but we want to ensure it runs
    println!("Part 1 real input result: {result}");
    assert!(result > 0);
}
