# Claude Project Notes - Advent of Code 2024

## Project Purpose

Learning Rust through AoC 2024 challenges. Focus on hands-on coding, not  
tutorials.

## Structure

- Rust workspace with separate packages for each day
- `shared/` package contains common utilities (input parsing, etc.)
- Run with: `cargo run -p dayXX`

## Implementation Status

- ✅ Day 1: Completed (distance between sorted lists)
- ❌ Days 2-25: Need to be implemented

## Next Steps for New Days

1. Create `dayXX/` directory
2. Copy `day01/Cargo.toml` and update package name
3. Create `dayXX/src/main.rs` with solution
4. Add `dayXX/input.txt` with puzzle input from AoC website
5. Test with example first, then run on real input

## Setup Notes

- When converting single package → workspace, remove leftover root `src/` folder
- Workspace members auto-detected by `members = ["day*", "shared"]` pattern

### Rust-Analyzer Settings (Beginner-Optimized)

Focused configuration for learning Rust without information overload:

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

**Future Additions (After Month 1):**

- Type hints, reborrow hints, expression adjustments (when comfortable with ownership)
- Advanced inlay hints (closures, discriminants, binding modes)

## Future: GitHub + Pre-commit Setup

1. Make this project a GitHub repository
2. Set up pre-commit hooks with linting (markdownlint-cli2, taplo)
3. Hooks run automatically on commit to enforce code quality
