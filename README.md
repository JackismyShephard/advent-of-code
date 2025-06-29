# Advent of Code 2024 - Rust Learning

Learning Rust through AoC 2024 challenges!

## Structure

- `day01/`, `day02/`, etc. - Individual day solutions
- `shared/` - Common utilities for input parsing
- Each day is a separate Rust package in the workspace

## Running Solutions

```bash
# Run day 1
cargo run -p day01

# Run any day
cargo run -p dayXX
```

## Adding New Days

1. Create `dayXX/` directory
2. Copy `day01/Cargo.toml` and update the name
3. Create `dayXX/src/main.rs` with your solution
4. Add input file as `dayXX/input.txt`

## Current Status

- ✅ Day 1: Distance calculation between two sorted lists (result: 11 for example)
