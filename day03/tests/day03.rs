use day03::{
    extract_enabled_mul_instructions, extract_mul_instructions, solve_part1, solve_part2,
    EXAMPLE_INPUT, EXAMPLE_INPUT_PART2,
};

#[test]
fn test_extract_mul_instructions_example() {
    let instructions = extract_mul_instructions(EXAMPLE_INPUT).unwrap();
    assert_eq!(instructions, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
}

#[test]
fn test_extract_mul_instructions_edge_cases() {
    // Test invalid formats that should be ignored
    let memory = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 ) mul[3,7] mul(123,456)";
    let instructions = extract_mul_instructions(memory).unwrap();
    assert_eq!(instructions, vec![(123, 456)]); // Only the last one is valid

    // Test empty input
    assert_eq!(extract_mul_instructions("").unwrap(), vec![]);

    // Test no valid instructions
    assert_eq!(
        extract_mul_instructions("no mul instructions here").unwrap(),
        vec![]
    );

    // Test boundary cases for 1-3 digits
    let memory2 = "mul(1,2) mul(12,34) mul(123,456) mul(1234,5) mul(1,2345)";
    let instructions2 = extract_mul_instructions(memory2).unwrap();
    assert_eq!(instructions2, vec![(1, 2), (12, 34), (123, 456)]); // 4-digit numbers should be ignored
}

#[test]
fn test_solve_part1_example() {
    let result = solve_part1(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 161); // 2*4 + 5*5 + 11*8 + 8*5 = 8 + 25 + 88 + 40 = 161
}

#[test]
fn test_solve_part1_simple() {
    assert_eq!(solve_part1("mul(2,3)").unwrap(), 6);
    assert_eq!(solve_part1("mul(10,10)").unwrap(), 100);
    assert_eq!(solve_part1("mul(1,1)mul(2,2)mul(3,3)").unwrap(), 14); // 1 + 4 + 9 = 14
}

#[test]
fn test_solve_part1_no_valid_instructions() {
    assert_eq!(solve_part1("no valid instructions").unwrap(), 0);
    assert_eq!(solve_part1("mul(4* mul[3,7] mul ( 2 , 4 )").unwrap(), 0);
}

#[test]
fn test_part1_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part1(&input).unwrap();

    assert_eq!(result, 167650499); // Actual result from puzzle input
}

#[test]
fn test_extract_enabled_mul_instructions_example() {
    let instructions = extract_enabled_mul_instructions(EXAMPLE_INPUT_PART2).unwrap();
    assert_eq!(instructions, vec![(2, 4), (8, 5)]);
}

#[test]
fn test_extract_enabled_mul_instructions_enabled_by_default() {
    // Test that mul instructions are enabled by default
    let memory = "mul(3,4)don't()mul(5,6)";
    let instructions = extract_enabled_mul_instructions(memory).unwrap();
    assert_eq!(instructions, vec![(3, 4)]);
}

#[test]
fn test_extract_enabled_mul_instructions_state_changes() {
    // Test multiple state changes
    let memory = "mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)";
    let instructions = extract_enabled_mul_instructions(memory).unwrap();
    assert_eq!(instructions, vec![(1, 1), (3, 3), (5, 5)]);
}

#[test]
fn test_extract_enabled_mul_instructions_no_state_changes() {
    // Test when there are no do()/don't() instructions
    let memory = "mul(1,2)mul(3,4)mul(5,6)";
    let instructions = extract_enabled_mul_instructions(memory).unwrap();
    assert_eq!(instructions, vec![(1, 2), (3, 4), (5, 6)]);
}

#[test]
fn test_extract_enabled_mul_instructions_only_disabled() {
    // Test when all mul instructions are disabled
    let memory = "don't()mul(1,2)mul(3,4)mul(5,6)";
    let instructions = extract_enabled_mul_instructions(memory).unwrap();
    assert_eq!(instructions, vec![]);
}

#[test]
fn test_extract_enabled_mul_instructions_empty_input() {
    assert_eq!(extract_enabled_mul_instructions("").unwrap(), vec![]);
}

#[test]
fn test_solve_part2_example() {
    let result = solve_part2(EXAMPLE_INPUT_PART2).unwrap();
    assert_eq!(result, 48); // 2*4 + 8*5 = 8 + 40 = 48
}

#[test]
fn test_solve_part2_simple_cases() {
    // Test simple enabled case
    assert_eq!(solve_part2("mul(2,3)").unwrap(), 6);

    // Test simple disabled case
    assert_eq!(solve_part2("don't()mul(2,3)").unwrap(), 0);

    // Test re-enabled case
    assert_eq!(solve_part2("don't()mul(2,3)do()mul(4,5)").unwrap(), 20);
}

#[test]
fn test_solve_part2_complex_state_changes() {
    // Test multiple state changes with calculations
    let memory = "mul(1,1)don't()mul(2,2)do()mul(3,3)don't()mul(4,4)do()mul(5,5)";
    let result = solve_part2(memory).unwrap();
    assert_eq!(result, 35); // 1*1 + 3*3 + 5*5 = 1 + 9 + 25 = 35
}

#[test]
fn test_part2_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part2(&input).unwrap();

    assert_eq!(result, 95846796); // Actual result from puzzle input
}
