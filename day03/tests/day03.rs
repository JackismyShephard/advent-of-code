use day03::{extract_mul_instructions, solve_part1, EXAMPLE_INPUT};

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
