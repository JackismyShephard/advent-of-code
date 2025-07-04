use anyhow::Result;
use day02::{solve_part1, solve_part1_functional};
use shared::benchmarking::{benchmark_function, print_benchmark_header, print_benchmark_result};
use shared::plotting::create_dual_algorithm_plot;

const SIZES: [usize; 6] = [100, 500, 1000, 2000, 5000, 10000];

fn main() -> Result<()> {
    let mut results = Vec::new();

    print_benchmark_header("Single Pass", "Functional");

    for &size in &SIZES {
        let input_string = generate_test_input(size);
        let samples = if size <= 1000 { 20 } else { 10 };

        let single_pass_time = benchmark_function(solve_part1, &input_string, samples);
        let functional_time = benchmark_function(solve_part1_functional, &input_string, samples);

        let speedup = functional_time / single_pass_time;
        results.push((size, single_pass_time, functional_time, speedup));

        print_benchmark_result(size, single_pass_time, functional_time, speedup);
    }

    create_dual_algorithm_plot(2, "Single-Pass", "Functional", &results)?;
    println!("✅ Benchmark completed successfully");
    Ok(())
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
