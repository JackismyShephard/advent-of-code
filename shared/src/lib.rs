use std::fs;
use anyhow::Result;

/// Read input file and return as string
pub fn read_input(day: u8) -> Result<String> {
    let filename = format!("day{:02}/input.txt", day);
    Ok(fs::read_to_string(filename)?)
}

/// Parse input into lines, filtering out empty lines
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.trim().is_empty()).collect()
}