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

- **Rust toolchain**:
  - Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Restart shell or run: `source ~/.cargo/env`

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

## Git Conventions

- Use concise, one-line commit messages
- Example: "Initial commit: Advent of Code 2024 Rust workspace setup"

## Code Quality Enforcement

### Pre-commit Hooks (Active)

**Purpose**: Automatically enforce code quality standards before each commit,
preventing broken or poorly formatted code from entering the repository.

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

**IMPORTANT - Never bypass hooks:**

- Never use `git commit --no-verify` unless absolutely necessary
- If hooks fail, fix the issues rather than bypassing them
- This maintains code quality and prevents technical debt

### Future Consideration: Migration to Just Command Runner

If pre-commit hooks become too intrusive or if we want more manual control
over quality checks, consider migrating to a `justfile`-based approach.
Replace automatic pre-commit hooks with manual `just check` commands.
Would require discipline to remember running checks before commits
