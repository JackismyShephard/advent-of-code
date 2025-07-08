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
- ✅ Day 2: Completed (Part 1: reactor safety report analysis, Part 2: Problem Dampener)
- ✅ Day 3: Completed (Part 1: corrupted memory mul instruction parsing,
  Part 2: do()/don't() conditional processing)
- 🔄 Day 4: Part 1 Complete (Part 1: XMAS word search in 2D grid, Part 2: TBD)

## Next Steps for New Days

1. Create `dayXX/` directory
2. Copy `day01/Cargo.toml` and update package name
3. Add `dayXX/description.txt` with problem description excerpts
4. Add `dayXX/input.txt` with puzzle input from AoC website
5. Create `dayXX/src/lib.rs` with core logic and EXAMPLE_INPUT constant
6. Create `dayXX/src/main.rs` with simple runner using lib functions
7. Create `dayXX/tests/dayXX.rs` with comprehensive tests (example + real input)
8. Test with example first, then run on real input
9. Optionally add `dayXX/benches/bench.rs` for performance benchmarks

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

## Development Workflow (For Claude)

- **When user says "remember" something: ALWAYS Add it to this CLAUDE.md file**

  - User instructions prefixed with "remember" should be documented here
  - This creates a persistent record of important project-specific guidance

- **after completing a task ALWAYS**

  - stage relevant files with `git add`
  - run pre-commit hooks to ensure code quality
  - DO NOT commit unless asked to do so.

## Common Linting/Type Errors and Fixes

**Remember these fixes for recurring issues:**

- **`clippy::uninlined_format_args`**: Use `{variable}` instead of `{}",
  variable` in format strings
  - ❌ `println!("Result: {}", result);`
  - ✅ `println!("Result: {result}");`

- **IMPORTANT: Always commit ALL files for each day solution:**

  - `dayXX/src/main.rs` - The solution code
  - `dayXX/Cargo.toml` - Dependencies
  - `dayXX/input.txt` - Personal puzzle input
  - `dayXX/description.txt` - Problem description excerpts
  - `CLAUDE.md` - Update implementation status
  - `README.md` - Update current status section
  - `Cargo.lock` - Dependency lock file

## Git conventions

- **ALWAYS follow these git commit message guidelines**

- Use concise, one-line commit messages
- Example: "Initial commit: Advent of Code 2024 Rust workspace setup"
- **NEVER include co-authoring or Claude references in commit messages**

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

## Research Methodology

### How to Conduct Proper Deep Research

**When user requests "ultrathink and research" or "deep research":**

#### Phase 1: Primary Source Investigation

- **RFCs & Language Issues**: Search rust-lang/rfcs for design decisions and rationale
- **Library Documentation**: Read actual API docs, not just descriptions
- **Community Forums**: Access Rust Internals, not just user forums
- **Version History**: Research when features were added and why

#### Phase 2: Cross-Domain Analysis

- **Multiple Domains**: Check game engines, scientific computing, image processing, etc..
- **Real Codebases**: Examine how major libraries solve problems
- **Performance Data**: Look for benchmarks and real-world measurements
- **Historical Context**: Understand evolution of approaches over time

#### Phase 3: Evidence Quality Assessment

- **Authoritative**: Language RFCs, core team discussions, library maintainer posts
- **Practical**: Stack Overflow with multiple upvotes, real project examples
- **Speculative**: Blog posts, opinions without backing data
- **Verify Access**: Confirm you can actually read sources, not just descriptions

#### Phase 4: Honest Reporting

- **Qualify Claims**: "Based on limited sources" vs "definitive consensus"
- **Cite Limitations**: Note 403 errors, paywalls, incomplete access
- **Multiple Perspectives**: Present competing viewpoints with evidence
- **Avoid Extrapolation**: Don't claim "industry standard" from one example

## Rust Signed / unsigned Problem

### Problem Statement

**The Core Tension:**

- **Array indexing**: Requires `usize` (unsigned)
  - language requirement to prevent negative array access
- **Arithmetic**: Often needs signed values.
- casting between signed and unsigned types leads to verbose code

**Why This is a Widespread Issue:**

- Acknowledged across entire Rust community as fundamental pain point
- Bjarne Stroustrup called unsigned indexing "a mistake in C++ STL design"
- **No consensus exists** - remains active debate (8+ page discussions)
- **Professional libraries** lean toward signed coordinates internally
- **Language team** committed to unsigned for safety

### Research-Backed Solution Patterns

#### Pattern 1: Modern Mixed Operations (Rust 1.66.0+)

```rust
// Official language solution, stabilized Dec 2022
let new_row = start_row.checked_add_signed(delta * i)?;
```

- **Evidence**: Stabilized in rust-lang/rust#87840 after community demand
- **Use Cases**: Cargo (CPU counts), io::SeekFrom operations
- **Pros**: Official support, safety guarantees, handles overflow
- **Cons**: Verbose Option handling, recent adoption

#### Pattern 2: All-Signed Internal (Professional Grid Libraries)

```rust
struct Coord { x: i32, y: i32 }  // grid_2d crate approach
```

- **Evidence**: Used by grid_2d, pathfinding, and other professional libraries
- **Rationale**: Clean arithmetic, convert to usize only at array boundaries  
- **Pros**: Simple math, proven in production, industry standard
- **Cons**: Conversion overhead at indexing points

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

## Performance Optimization & Profiling

**IMPORTANT**: Always use `--release` flag for performance-sensitive code

### Profiling Workflow

#### Step 1: Establish Baseline with Criterion

```bash
# Add Criterion to Cargo.toml [dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

# Create focused benchmarks (see day01/benches/day01_benchmarks.rs)
cargo bench
```

#### Step 2: Component-Level Analysis

Create separate benchmarks for each major component to identify bottlenecks:

```rust
// Example: Break down parsing vs sorting vs calculation
c.bench_function("parsing_only", |b| {
    b.iter(|| parse_input(black_box(&input)));
});

c.bench_function("sorting_only", |b| {
    b.iter(|| {
        let mut data = parsed_data.clone();
        data.sort();
        black_box(data);
    });
});
```

#### Step 3: Profile-Guided Optimization

Target the **biggest bottleneck first**

#### Step 4: Verify Improvements

```bash
# Compare original vs optimized implementations
cargo bench

# Look for consistent 1.2x+ improvements across multiple sizes
```

### What Generally Doesn't Work for AoC Problems

**Based on Day 1 experiments:**

1. **Parallelization** (0.0-0.7x speedup)
   - Thread overhead dominates for small datasets
   - Beneficial only at 500,000+ elements
   - AoC problems typically 100-10,000 elements

2. **Profile-Guided Optimization (PGO)** (0.96x speedup)
   - Helps with complex branching patterns
   - AoC code has simple, predictable control flow
   - Most time spent in stdlib functions already optimized

3. **Manual micro-optimizations** (1.0-1.1x speedup)
   - Loop unrolling: LLVM already optimizes
   - SIMD: Limited benefit for simple operations
   - Rust's stdlib is already highly optimized

### Optimization Priority (Proven Effective)

1. **Algorithm choice** - O(n) vs O(n²) (day01: 7.5x speedup HashMap vs naive)
2. **Data structure optimization** - HashMap vs Vec, pre-allocation
3. **Parsing optimization** - SIMD libraries for known formats (day01: 1.67x speedup)
4. **Compiler optimizations** - Always use `--release` mode

## Benchmarking and Plotting Architecture

### Research Summary

**After extensive research comparing plotting and benchmarking approaches, we established our final architecture:**

### **Plotting Decision: Custom Plotters**

- **Chosen Approach**: Custom plotters-based visualization
- **Key Finding**: Plotters provides equivalent visual quality to Criterion's gnuplot output with greater control and customization
- **Research Conclusion**: No significant functional differences between gnuplot and plotters backends in Criterion - the choice is purely about dependency management (external vs pure Rust)
- **Advantages**: Complete control over plot styling, logarithmic scaling, custom annotations, and integration with our benchmarking workflow

### **Benchmarking Decision: Criterion.rs**

- **Chosen Approach**: Criterion.rs for data collection and statistical analysis
- **Research Context**: Comprehensive comparison of Current Custom Setup vs Criterion vs Divan
- **Key Factors**:
  - **Integration**: Criterion provides superior JSON/CSV export for custom plotting integration
  - **Tooling**: Mature cargo bench support and programmatic data access
  - **Statistics**: Valuable statistical rigor (bootstrap sampling, confidence intervals, outlier detection)
  - **Ecosystem**: Industry standard with extensive documentation and community support

### **Final Architecture**

- **Data Collection**: Criterion.rs benchmarks with `harness = false` in Cargo.toml
- **Data Extraction**: JSON/CSV export from Criterion for custom analysis
- **Visualization**: Custom plotters-based plots with full control over styling and layout
- **Workflow**: `cargo bench` → extract data → generate custom plots
- **Benefits**: Combines Criterion's statistical rigor with our custom visualization capabilities

This hybrid approach maximizes both measurement accuracy and visualization quality while maintaining learning-focused simplicity.
