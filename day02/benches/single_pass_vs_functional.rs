use criterion::{criterion_group, criterion_main, Criterion};
use day02::{solve_part1, solve_part1_functional};
use shared::benchmarking::{
    create_criterion_benchmark, process_benchmark_results, run_dual_algorithm_benchmark, Algorithm,
    PlotConfig, TestConfig,
};

const SIZES: [usize; 6] = [100, 500, 1000, 2000, 5000, 10000];

/// Criterion benchmark with JSON extraction and co-located output
fn benchmark_algorithms(c: &mut Criterion) {
    let data_dir = "data";
    let group_name = "criterion";

    // Algorithm definitions
    let algorithm1 = Algorithm {
        name: "single_pass",
        function: solve_part1,
    };
    let algorithm2 = Algorithm {
        name: "functional",
        function: solve_part1_functional,
    };

    // Test configuration
    let test_config = TestConfig {
        sizes: &SIZES,
        generate_input: generate_test_input,
    };

    // Run the benchmark
    run_dual_algorithm_benchmark(c, group_name, &algorithm1, &algorithm2, &test_config);

    // Process results and generate outputs
    let plot_config = PlotConfig {
        filename: "single_pass_vs_functional.svg",
        title: "Day 2: Single-Pass vs Functional Algorithm Performance",
        algorithm1_name: "Single-Pass (Optimized)",
        algorithm2_name: "Functional (Iterator Approach)",
    };

    process_benchmark_results(
        data_dir,
        group_name,
        &algorithm1,
        &algorithm2,
        &plot_config,
        &test_config,
    );
}

/// Generates synthetic test input for performance benchmarking.
///
/// Creates simple deterministic reports with realistic safe/unsafe distribution.
/// 70% safe reports, 30% unsafe for representative benchmarking.
///
/// # Parameters
/// * `count` - Number of reports to generate
///
/// # Returns
/// String representation of reports in input format
fn generate_test_input(count: usize) -> String {
    (0..count)
        .map(|i| {
            let base = (i % 50) + 1;
            let length = (i % 3) + 4; // Simple length variation: 4-6

            if i % 10 < 7 {
                // 70% safe reports
                let ascending = i % 2 == 0;
                (0..length)
                    .map(|j| {
                        let value = if ascending {
                            base + j
                        } else {
                            base + length - 1 - j
                        };
                        value.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                // 30% unsafe reports - simple large jump pattern
                (0..length)
                    .map(|j| {
                        let value = match j {
                            0 => base,
                            1 => base + 1,
                            2 => base + 8, // Jump of 7 (unsafe)
                            _ => base + 8 + j - 2,
                        };
                        value.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

criterion_group!(
    name = benches;
    config = create_criterion_benchmark("data");
    targets = benchmark_algorithms
);
criterion_main!(benches);
