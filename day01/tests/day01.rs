use day01::{parse_input, solve_part1, solve_part2, EXAMPLE_INPUT};

#[test]
fn test_part1_example() {
    let result = solve_part1(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 11);
}

#[test]
fn test_part2_example() {
    let result = solve_part2(EXAMPLE_INPUT).unwrap();
    assert_eq!(result, 31);
}

#[test]
fn test_parse_input() {
    let (left, right) = parse_input(EXAMPLE_INPUT).unwrap();
    assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
}

#[test]
fn test_part1_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part1(&input).unwrap();
    assert_eq!(result, 1603498);
}

#[test]
fn test_part2_real_input() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Failed to read input.txt - make sure it exists");
    let result = solve_part2(&input).unwrap();
    assert_eq!(result, 25574739);
}
