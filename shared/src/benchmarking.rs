use std::hint::black_box;
use std::time::Instant;

/// Number of warmup iterations to perform before benchmarking
pub const WARMUP_ITERATIONS: usize = 3;

/// Prints the header for benchmark results table.
///
/// # Parameters
/// * `algorithm1_name` - Name of the first algorithm for column header
/// * `algorithm2_name` - Name of the second algorithm for column header
pub fn print_benchmark_header(algorithm1_name: &str, algorithm2_name: &str) {
    println!("Size\t{algorithm1_name} (μs)\t{algorithm2_name} (μs)\tSpeedup");
    println!(
        "----\t{}--------\t{}--------\t-------",
        "-".repeat(algorithm1_name.len()),
        "-".repeat(algorithm2_name.len())
    );
}

/// Benchmarks a function by measuring its median execution time.
///
/// Performs warmup iterations to stabilize CPU performance, then takes multiple
/// timing samples and returns the median to reduce noise from system interference.
///
/// # Parameters
/// * `algorithm_fn` - The function to benchmark (takes input string, returns result)
/// * `test_input` - The input data to run the algorithm on
/// * `sample_count` - Number of timing samples to take for statistical accuracy
///
/// # Returns
/// Median execution time in nanoseconds
pub fn benchmark_function<F, R>(algorithm_fn: F, test_input: &str, sample_count: usize) -> f64
where
    F: Fn(&str) -> R,
{
    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        let _ = black_box(algorithm_fn(black_box(test_input)));
    }

    let mut times: Vec<f64> = (0..sample_count)
        .map(|_| {
            let start = Instant::now();
            let _ = black_box(algorithm_fn(black_box(test_input)));
            start.elapsed().as_nanos() as f64
        })
        .collect();

    times.sort_by(|a, b| a.total_cmp(b));
    times[times.len() / 2]
}

/// Prints benchmark results in a formatted table.
///
/// # Parameters
/// * `size` - The input size for this benchmark run
/// * `time1` - Execution time of first algorithm in nanoseconds
/// * `time2` - Execution time of second algorithm in nanoseconds
/// * `speedup` - Speedup factor (time2 / time1)
pub fn print_benchmark_result(size: usize, time1: f64, time2: f64, speedup: f64) {
    let time1_us = time1 / 1000.0;
    let time2_us = time2 / 1000.0;

    println!("{size}\t{time1_us:.2}\t\t{time2_us:.2}\t\t{speedup:.1}x");
}
