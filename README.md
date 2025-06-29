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
- **TOML files**: Formatting and linting (taplo)
- **Markdown files**: Will be added in future updates
