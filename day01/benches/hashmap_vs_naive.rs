use anyhow::Result;
use day01::{solve_part2, solve_part2_naive};
use shared::benchmarking::{benchmark_function, print_benchmark_header, print_benchmark_result};
use shared::plotting::create_dual_algorithm_plot;

const SIZES: [usize; 6] = [500, 1000, 2000, 5000, 8000, 12000];

fn main() -> Result<()> {
    let mut results = Vec::new();

    print_benchmark_header("Hashmap", "Naive");

    for &size in &SIZES {
        let input = generate_test_input(size);
        let samples = if size <= 2000 { 10 } else { 5 };

        let hashmap_time = benchmark_function(solve_part2, &input, samples);
        let naive_time = benchmark_function(solve_part2_naive, &input, samples);

        let speedup = naive_time / hashmap_time;
        results.push((size, hashmap_time, naive_time, speedup));

        print_benchmark_result(size, hashmap_time, naive_time, speedup);
    }

    create_dual_algorithm_plot(1, "O(n) Hashmap", "O(n²) Naive", &results)?;

    Ok(())
}

/// Generates synthetic test input for performance benchmarking.
///
/// Creates deterministic but varied integer pairs using modular arithmetic
/// to ensure realistic distribution while maintaining reproducibility.
///
/// # Parameters
/// * `size` - Number of input pairs to generate for the test dataset
///
/// # Returns
/// Newline-separated string of integer pairs in "left right" format
fn generate_test_input(size: usize) -> String {
    (0..size)
        .map(|i| format!("{} {}", (i % 9999) + 1, ((i * 7) % 9999) + 1))
        .collect::<Vec<String>>()
        .join("\n")
}
