# Claude Project Notes - Advent of Code 2024

## Project Purpose

Learning Rust through AoC 2024 challenges. Focus on hands-on coding, not  
tutorials.

## Structure

- Rust workspace with separate packages for each day
- `shared/` package contains common utilities (input parsing, etc.)
- Run with: `cargo run -p dayXX`
- Test with: `cargo test -p dayXX`
- Benchmark with: `cargo bench -p dayXX`

## Implementation Status

- ✅ Day 1: Completed (Part 1: distance between sorted lists, Part 2: similarity score)
- ❌ Days 2-25: Need to be implemented

## Next Steps for New Days

1. Create `dayXX/` directory
2. Copy `day01/Cargo.toml` and update package name
3. Create `dayXX/src/lib.rs` with core logic and EXAMPLE_INPUT constant
4. Create `dayXX/src/main.rs` with simple runner using lib functions
5. Create `dayXX/tests/dayXX.rs` with comprehensive tests (example + real input)
6. Add `dayXX/input.txt` with puzzle input from AoC website
7. Add `dayXX/description.txt` with problem description excerpts
8. Test with example first, then run on real input

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

## Development Workflow (For Claude)

- **Always commit changes after completing a task**

  - Stage relevant files with `git add`
  - Create descriptive commit message
  - Push changes to remote repository
  - This ensures progress is saved and quality hooks are enforced

- **IMPORTANT: Always commit ALL files for each day solution:**

  - `dayXX/src/main.rs` - The solution code
  - `dayXX/Cargo.toml` - Dependencies
  - `dayXX/input.txt` - Personal puzzle input
  - `dayXX/description.txt` - Problem description excerpts
  - `CLAUDE.md` - Update implementation status
  - `README.md` - Update current status section
  - `Cargo.lock` - Dependency lock file

- **When user says "remember" something: Add it to this CLAUDE.md file**

  - User instructions prefixed with "remember" should be documented here
  - This creates a persistent record of important project-specific guidance

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
- **Tests**: All unit and integration tests must pass (cargo test)
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

## Documentation Best Practices

### Rust Documentation Standards

Following Rust ecosystem conventions for writing high-quality documentation:

#### Standard Documentation Sections (in order)

1. **Brief description** - What the function does (first paragraph)
2. **Detailed explanation** - How it works, algorithms used, performance characteristics
3. **`# Parameters`** - **Explicit semantic documentation of what each
   parameter represents**
4. **`# Returns`** - **Explicit description of return value meaning and units**
5. **`# Errors`** - When function returns `Result<T, E>` (fallible functions)
6. **`# Panics`** - When function can panic (if applicable)
7. **`# Examples`** - Code examples showing usage (most important!)
   - Use `# use` to import necessary items
8. **`# Safety`** - For unsafe functions only

#### Project-Specific Enhancement: Explicit Semantic Documentation

**Philosophy**: While Rust's type system is excellent, it doesn't always convey
the *semantic meaning* of parameters and return values. We enhance standard Rust
docs with explicit parameter and return documentation.

#### What NOT to include

- Redundant type information - Function signature is self-documenting for types
- Overly verbose descriptions - Keep semantic descriptions concise but complete

### Generating Documentation

```bash
# Generate and open documentation in browser
cargo doc --no-deps --open -p dayXX

# Generate documentation for all packages
cargo doc --no-deps
```

#### WSL-Specific Documentation Viewing

If `cargo doc --open` fails with permission errors in WSL:

```bash
# Fix permissions
sudo chown -R $USER:$USER target/

# Install wslu and use wslview
sudo apt install wslu
cargo doc --no-deps
wslview target/doc/dayXX/index.html
```
