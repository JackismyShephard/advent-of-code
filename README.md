# Advent of Code 2024 - Rust Learning

Learning Rust through AoC 2024 challenges!

## Structure

- `day01/`, `day02/`, etc. - Individual day solutions
- `shared/` - Common utilities for input parsing
- Each day is a separate Rust package in the workspace

## Running Solutions

```bash
# Run solution
cargo run -p dayXX

# Run tests
cargo test -p dayXX

# Run benchmarks
cargo bench -p dayXX
```

## Current Status

- ✅ Day 1: Complete
  - Part 1: Distance calculation between sorted lists
    (result: 11 for example, 1,603,498 for real input)
  - Part 2: Similarity score using frequency maps
    (result: 31 for example, 25,574,739 for real input)
  - Performance benchmark: O(n) vs O(n²) comparison with visual graph generation

- ✅ Day 2: Complete
  - Part 1: Reactor safety report analysis
    (result: 2 for example, 686 for real input)
  - Part 2: Problem Dampener - allows removing one level to make unsafe reports safe
    (result: 4 for example, 717 for real input)
  
- ✅ Day 3: Complete
  - Part 1: Corrupted memory mul instruction parsing
    (result: 161 for example, 190,604,937 for real input)
  - Part 2: do()/don't() conditional processing
    (result: 48 for example, 82,857,512 for real input)

- 🔄 Day 4: Part 1 Complete
  - Part 1: XMAS word search in 2D grid (8 directions)
    (result: 18 for example, TBD for real input)
  - Part 2: TBD

## Adding New Days

1. Create `dayXX/` directory
2. Add problem description as `dayXX/description.txt`
3. Add input file as `dayXX/input.txt`
4. Copy `day01/Cargo.toml` and update the name
5. Create `dayXX/src/lib.rs` with core logic and EXAMPLE_INPUT constant
6. Create `dayXX/src/main.rs` with simple runner using lib functions
7. Create `dayXX/tests/dayXX.rs` with comprehensive tests (example + real input)
8. optionally add `dayXX/benches/bench.rs` for performance benchmarks

## Performance Analysis

Some days include performance benchmarks comparing different algorithmic approaches:

- **Day 1**: Run `cargo bench -p day01` to generate `performance_comparison.svg`
- Shows performance scaling between optimized hashmap approach vs naive nested loops
- Demonstrates clear O(n) vs O(n²) performance differences with speedup factors
- **Day 2**: Run `cargo bench -p day02` for micro-benchmarks
- Compares different safety checking approaches and Problem Dampener implementations

## Setup Notes

- **Rust toolchain**:
  - Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Restart shell or run: `source ~/.cargo/env`

### Rust-Analyzer Settings (Beginner-Optimized)

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.diagnostics.styleLints.enable": true,
  "rust-analyzer.completion.fullFunctionSignatures.enable": true,
  "rust-analyzer.testExplorer": true,
  "rust-analyzer.inlayHints.lifetimeElisionHints.enable": "skip_trivial",
  "rust-analyzer.imports.granularity.enforce": true,
  "rust-analyzer.restartServerOnConfigChange": true
}
```

## Code Quality Enforcement

**Setup Instructions:**

```bash
# Download latest pre-commit (avoids nodeenv issues with apt version)
curl -LO https://github.com/pre-commit/pre-commit/releases/download/v4.2.0/pre-commit-4.2.0.pyz

# Install hooks in repository (one-time setup)
python3 pre-commit-4.2.0.pyz install

# Test hooks manually (optional)
python3 pre-commit-4.2.0.pyz run --all-files
```

**What the hooks check:**

- **Rust code**: Formatting (rustfmt) and linting (clippy with strict warnings)
- **Tests**: All unit and integration tests must pass (cargo test)
- **TOML files**: Formatting and linting (taplo)
- **Markdown files**: Formatting and linting (markdownlint)

## Documentation

### Generating Documentation

```bash

# Generate and open documentation in browser
cargo doc --no-deps --open -p dayXX

# Generate documentation for all packages
cargo doc --no-deps
```

### WSL-Specific Documentation Viewing

If `cargo doc --open` fails with permission errors in WSL:

```bash
# Fix permissions
sudo chown -R $USER:$USER target/

# Install wslu and use wslview
sudo apt install wslu
cargo doc --no-deps
wslview target/doc/dayXX/index.html
```
