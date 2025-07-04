use day02::{
    is_safe, is_safe_functional, is_safe_with_dampener, parse_input, solve_part1,
    solve_part1_functional, solve_part2, EXAMPLE_INPUT,
};

#[test]
fn test_parse_input() {
    let input = "1 2 3\n4 5 6";
    let reports = parse_input(input).unwrap();
    assert_eq!(reports, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_is_safe_examples() {
    // Test cases from the problem description
    assert!(is_safe(&[7, 6, 4, 2, 1])); // Safe: decreasing by 1 or 2
    assert!(!is_safe(&[1, 2, 7, 8, 9])); // Unsafe: 2->7 is increase of 5
    assert!(!is_safe(&[9, 7, 6, 2, 1])); // Unsafe: 6->2 is decrease of 4
    assert!(!is_safe(&[1, 3, 2, 4, 5])); // Unsafe: 1->3 increasing, 3->2 decreasing
    assert!(!is_safe(&[8, 6, 4, 4, 1])); // Unsafe: 4->4 no change
    assert!(is_safe(&[1, 3, 6, 7, 9])); // Safe: increasing by 1, 2, or 3
}

#[test]
fn test_is_safe_edge_cases() {
    assert!(is_safe(&[])); // Empty report is safe
    assert!(is_safe(&[1])); // Single level is safe
    assert!(is_safe(&[1, 2])); // Two levels, valid difference
    assert!(!is_safe(&[1, 5])); // Two levels, invalid difference (4)
    assert!(!is_safe(&[5, 5])); // Two levels, no change
}

#[test]
fn test_solve_part1_example() {
    let result = solve_part1(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 2); // From problem description: 2 reports are safe
}

#[test]
fn test_solve_part1_custom() {
    let input = "1 2 3\n5 4 3 2\n1 1 1";
    let result = solve_part1(input).unwrap();
    assert_eq!(result, 2); // First two are safe, third has no changes
}

#[test]
fn test_is_safe_functional_examples() {
    // Test cases from the problem description - should match is_safe exactly
    assert!(is_safe_functional(&[7, 6, 4, 2, 1])); // Safe: decreasing by 1 or 2
    assert!(!is_safe_functional(&[1, 2, 7, 8, 9])); // Unsafe: 2->7 is increase of 5
    assert!(!is_safe_functional(&[9, 7, 6, 2, 1])); // Unsafe: 6->2 is decrease of 4
    assert!(!is_safe_functional(&[1, 3, 2, 4, 5])); // Unsafe: 1->3 increasing, 3->2 decreasing
    assert!(!is_safe_functional(&[8, 6, 4, 4, 1])); // Unsafe: 4->4 no change
    assert!(is_safe_functional(&[1, 3, 6, 7, 9])); // Safe: increasing by 1, 2, or 3
}

#[test]
fn test_is_safe_functional_edge_cases() {
    assert!(is_safe_functional(&[])); // Empty report is safe
    assert!(is_safe_functional(&[1])); // Single level is safe
    assert!(is_safe_functional(&[1, 2])); // Two levels, valid difference
    assert!(!is_safe_functional(&[1, 5])); // Two levels, invalid difference (4)
    assert!(!is_safe_functional(&[5, 5])); // Two levels, no change
}

#[test]
fn test_functional_vs_single_pass_equivalence() {
    // Test that both approaches produce identical results on the example
    let reports = parse_input(EXAMPLE_INPUT).unwrap();
    for report in &reports {
        assert_eq!(
            is_safe(report),
            is_safe_functional(report),
            "Mismatch for report: {report:?}"
        );
    }
}

#[test]
fn test_solve_part1_functional_example() {
    let result = solve_part1_functional(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 2); // Should match solve_part1
}

#[test]
fn test_solve_part1_functional_equivalence() {
    // Test that both solve functions produce identical results
    assert_eq!(
        solve_part1(EXAMPLE_INPUT).unwrap(),
        solve_part1_functional(EXAMPLE_INPUT).unwrap()
    );
}

#[test]
fn test_part1_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part1(&input).unwrap();
    assert_eq!(result, 686); // Actual result from puzzle input
}

#[test]
fn test_part1_functional_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part1_functional(&input).unwrap();
    assert_eq!(result, 686); // Actual result from puzzle input
}

#[test]
fn test_is_safe_with_dampener_examples() {
    // Test cases from Part 2 problem description
    assert!(is_safe_with_dampener(&[7, 6, 4, 2, 1])); // Safe without removing any level
    assert!(!is_safe_with_dampener(&[1, 2, 7, 8, 9])); // Unsafe regardless of removal
    assert!(!is_safe_with_dampener(&[9, 7, 6, 2, 1])); // Unsafe regardless of removal
    assert!(is_safe_with_dampener(&[1, 3, 2, 4, 5])); // Safe by removing second level (3)
    assert!(is_safe_with_dampener(&[8, 6, 4, 4, 1])); // Safe by removing third level (4)
    assert!(is_safe_with_dampener(&[1, 3, 6, 7, 9])); // Safe without removing any level
}

#[test]
fn test_is_safe_with_dampener_edge_cases() {
    assert!(is_safe_with_dampener(&[])); // Empty report is safe
    assert!(is_safe_with_dampener(&[1])); // Single level is safe
    assert!(is_safe_with_dampener(&[1, 2])); // Two levels, valid difference
    assert!(is_safe_with_dampener(&[1, 5])); // Two levels, can remove one to make safe
    assert!(is_safe_with_dampener(&[5, 5])); // Two levels, can remove one to make safe
    assert!(is_safe_with_dampener(&[1, 2, 3])); // Already safe
    assert!(is_safe_with_dampener(&[1, 4, 3])); // Can remove 4 to make 1->3 safe
    assert!(!is_safe_with_dampener(&[1, 5, 9, 13])); // All jumps too large, can't fix with one removal
}

#[test]
fn test_solve_part2_example() {
    let result = solve_part2(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 4); // From Part 2 problem description: 4 reports are safe with dampener
}

#[test]
fn test_solve_part2_custom() {
    let input = "1 2 3\n1 5 2\n10 8 6 4\n1 1 1 1";
    let result = solve_part2(input).unwrap();
    // 1 2 3: safe already
    // 1 5 2: can remove 5 to get 1->2 (safe)
    // 10 8 6 4: safe already (decreasing by 2)
    // 1 1 1 1: can remove any one 1 to get repeated values, but still unsafe (no change)
    assert_eq!(result, 3);
}

#[test]
fn test_part2_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part2(&input).unwrap();
    assert_eq!(result, 717); // Actual result from puzzle input
}
