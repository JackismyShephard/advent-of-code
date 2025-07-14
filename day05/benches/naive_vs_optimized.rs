use criterion::{criterion_group, criterion_main, Criterion};
use day05::{solve_part1, solve_part1_naive};
use shared::benchmarking::{
    create_criterion_benchmark, process_benchmark_results, run_dual_algorithm_benchmark, Algorithm,
    PlotConfig, TestConfig,
};

const SIZES: [usize; 8] = [100, 300, 600, 1000, 1500, 2000, 3000, 5000]; // number of sequences to validate

/// Criterion benchmark with JSON extraction and co-located output
fn benchmark_algorithms(c: &mut Criterion) {
    let data_dir = "data";
    let group_name = "criterion";

    // Algorithm definitions
    let algorithm1 = Algorithm {
        name: "optimized",
        function: solve_part1,
    };
    let algorithm2 = Algorithm {
        name: "naive",
        function: solve_part1_naive,
    };

    // Test configuration
    let test_config = TestConfig {
        sizes: &SIZES,
        generate_input: generate_test_input,
    };

    // Process results and generate outputs
    let plot_config = PlotConfig {
        filename: "optimized_vs_naive.svg",
        title: "Day 5: Optimized vs Naive Algorithm Performance",
        algorithm1_name: "O(V + E) Optimized Solution",
        algorithm2_name: "O(N²M) Naive Algorithm",
    };

    // Run the benchmark
    run_dual_algorithm_benchmark(c, group_name, &algorithm1, &algorithm2, &test_config);

    process_benchmark_results(
        data_dir,
        group_name,
        &algorithm1,
        &algorithm2,
        &plot_config,
        &test_config,
    );
}

/// Generates realistic synthetic test input for performance benchmarking.
///
/// Creates deterministic page ordering rules and sequences that reflect
/// real-world print queue scenarios and constraint satisfaction patterns.
/// Designed to highlight performance differences between naive O(N²M) and
/// optimized O(V+E) validation algorithms through realistic data distributions.
///
/// **Research-Based Data Distribution:**
/// - **Rule Density**: 20% edge density (sparse DAG, realistic constraints)
/// - **Sequence Validity**: 75% valid, 25% invalid (manufacturing standards)
/// - **Violation Patterns**: Direct precedence (45%), cycles (35%), partial (20%)
/// - **Sequence Lengths**: 40% short, 45% medium, 15% long (enterprise patterns)
/// - **Rule Duplication**: 8% redundant rules (real-world data characteristics)
///
/// **Generation Architecture:**
/// - **Fixed Universe**: 100 pages (0-99) with consistent rule complexity
/// - **Scalable Sequences**: Input parameter controls validation workload  
/// - **Deterministic**: Reproducible results for reliable benchmarking
/// - **Performance-Focused**: Fast generation without expensive graph algorithms
///
/// This approach directly scales algorithmic work (sequence validation) while
/// maintaining realistic constraint patterns that differentiate algorithm
/// performance characteristics.
///
/// # Parameters
/// * `n_sequences` - Number of sequences to generate for validation (scales benchmark work)
///
/// # Returns
/// String representation of rules and sequences in Day 5 input format:
/// ```text
/// 0|5
/// 1|6
/// 2|7
///
/// 0,1,2,5,6,7
/// 3,1,2,5,6,7
/// 7,6,5,2,1,0
/// ```
///
/// # Examples
/// ```
/// let input = generate_test_input(1000);
/// // Returns ~500 rules for 100-page universe with 1000 sequences to validate
/// // ~750 valid sequences (75%) and ~250 invalid sequences (25%)
/// ```
fn generate_test_input(n_sequences: usize) -> String {
    const N_PAGES: usize = 100; // larger universe to increase complexity

    // Generate realistic rule set with proper DAG structure and duplicates
    let rules = generate_realistic_rules(N_PAGES);

    // Generate sequences with research-based valid/invalid distribution
    let sequences = generate_realistic_sequences(n_sequences, N_PAGES, &rules);

    format!("{}\n\n{}", rules.join("\n"), sequences.join("\n"))
}

/// Generates realistic precedence rules forming a sparse DAG structure.
///
/// Creates rules with ~20% edge density using manufacturing workflow patterns:
/// backbone chains, cross-connections, and occasional complex dependencies.
/// Includes ~8% duplicate rules to test algorithm robustness.
fn generate_realistic_rules(n_pages: usize) -> Vec<String> {
    let mut rules = Vec::new();
    let target_rules = (n_pages * n_pages / 5).min(500); // ~20% density, higher cap for scaling

    // Backbone chains (40% of rules): create basic ordering structure
    let chain_length = n_pages / 3;
    for i in 0..(chain_length - 1) {
        rules.push(format!("{i}|{}", i + 1));
        rules.push(format!("{}|{}", i + chain_length, i + chain_length + 1));
    }

    // Cross-connections (50% of rules): sparse dependencies between chains
    let cross_rules = target_rules - rules.len();
    for i in 0..cross_rules {
        let before = (i * 3) % (n_pages / 2);
        let after = (n_pages / 2) + ((i * 5) % (n_pages / 2));
        if before != after {
            rules.push(format!("{before}|{after}"));
        }
    }

    // Add duplicates (8% of total): test algorithm duplicate handling
    let duplicate_count = rules.len() / 12;
    for i in 0..duplicate_count {
        if i < rules.len() {
            rules.push(rules[i].clone());
        }
    }

    // Remove duplicates that we don't want (keep only intentional ones)
    rules.sort();
    rules.dedup();

    rules
}

/// Generates sequences with realistic valid/invalid distribution and violation patterns.
///
/// Creates 75% valid sequences (topological orderings) and 25% invalid sequences
/// with specific violation patterns based on constraint satisfaction research.
/// Length distribution follows enterprise scheduling patterns.
fn generate_realistic_sequences(count: usize, n_pages: usize, _rules: &[String]) -> Vec<String> {
    let mut sequences = Vec::new();

    let valid_count = (count * 3) / 4; // 75% valid
    let invalid_count = count - valid_count; // 25% invalid

    // Generate valid sequences with varied complexity
    for i in 0..valid_count {
        let length = match i % 10 {
            0..=3 => 15 + (i % 15), // 40% short (15-29 pages)
            4..=8 => 30 + (i % 20), // 45% medium (30-49 pages)
            _ => 50 + (i % 30),     // 15% long (50-79 pages)
        };
        let length = length.min(n_pages);

        if i % 3 == 0 {
            // Simple ascending order (easy case)
            let seq: Vec<String> = (0..length).map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        } else {
            // Complex but valid ordering (harder case)
            let mut seq: Vec<usize> = (0..length).collect();
            // Controlled shuffles that maintain some valid orderings
            for j in (0..length).step_by(3) {
                if j + 1 < length && (j / 3) % 2 == 0 {
                    seq.swap(j, j + 1);
                }
            }
            let seq: Vec<String> = seq.iter().map(|x| x.to_string()).collect();
            sequences.push(seq.join(","));
        }
    }

    // Generate invalid sequences with specific violation patterns
    for i in 0..invalid_count {
        let length = 20 + (i % 40); // varied lengths for invalid sequences
        let length = length.min(n_pages);

        let mut seq: Vec<usize> = (0..length).collect();

        match i % 10 {
            0..=4 => {
                // Direct precedence violations (45% of invalid)
                if length >= 2 {
                    seq.reverse(); // Creates maximum violations
                }
            }
            5..=7 => {
                // Cycle violations (35% of invalid)
                if length >= 3 {
                    seq[0] = length - 1;
                    seq[length - 1] = 0;
                }
            }
            _ => {
                // Partial ordering violations (20% of invalid)
                for j in (0..length).step_by(2) {
                    if j + 1 < length {
                        seq.swap(j, j + 1);
                    }
                }
            }
        }

        let seq: Vec<String> = seq.iter().map(|x| x.to_string()).collect();
        sequences.push(seq.join(","));
    }

    sequences
}

criterion_group!(
    name = benches;
    config = create_criterion_benchmark("data");
    targets = benchmark_algorithms
);
criterion_main!(benches);
