use anyhow::{Context, Result};
use criterion::{BenchmarkId, Criterion};
use serde_json::Value;
use std::hint::black_box;
use std::path::Path;
use std::time::Duration;

/// Benchmark results type: (size, algorithm1_time, algorithm2_time, speedup)
pub type BenchmarkResults = Vec<(usize, f64, f64, f64)>;

/// Algorithm definition pairing name with implementation.
///
/// Groups semantically related parameters: an algorithm's identifier and its function.
pub struct Algorithm<'a, F> {
    pub name: &'a str,
    pub function: F,
}

/// Test configuration for benchmark execution.
///
/// Groups semantically related parameters: test sizes and input generation method.
pub struct TestConfig<'a, G> {
    pub sizes: &'a [usize],
    pub generate_input: G,
}

/// Plot configuration for benchmark visualization.
///
/// Contains only display-related parameters for plot generation.
/// Data extraction parameters (Criterion folder names) are handled separately.
///
/// # Parameters
/// * `filename` - Output SVG filename (should be filesystem-safe)
/// * `title` - Human-readable title displayed on the plot (can contain any characters)
/// * `algorithm1_name` - Human-readable name for algorithm 1 in plot legend
/// * `algorithm2_name` - Human-readable name for algorithm 2 in plot legend
///
/// # Examples
/// ```
/// # use shared::benchmarking::PlotConfig;
/// let config = PlotConfig {
///     filename: "naive_vs_hashmap.svg",
///     title: "Algorithm Performance Comparison",
///     algorithm1_name: "O(n²) Naive Algorithm",      // Can have Unicode, spaces, etc.
///     algorithm2_name: "O(n) HashMap Solution",      // Can have Unicode, spaces, etc.
/// };
/// ```
pub struct PlotConfig<'a> {
    pub filename: &'a str,
    pub title: &'a str,
    pub algorithm1_name: &'a str,
    pub algorithm2_name: &'a str,
}

/// Creates a Criterion instance optimized for fast benchmarking.
///
/// Configures reduced timing parameters for faster execution while maintaining
/// statistical accuracy suitable for AoC problems.
///
/// # Parameters
/// * `data_dir` - Directory path where benchmark data will be stored
///
/// # Returns
/// Configured Criterion instance with fast settings (500ms warmup, 2000ms measurement, 20 samples)
///
/// # Examples
///
/// ```
/// # use shared::benchmarking::create_criterion_benchmark;
/// let mut c = create_criterion_benchmark("day01/data/criterion");
/// // Use the criterion instance for benchmarking...
/// ```
pub fn create_criterion_benchmark(data_dir: &str) -> Criterion {
    Criterion::default()
        .output_directory(Path::new(data_dir))
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_millis(2000))
        .sample_size(20)
}

/// Benchmarks two algorithms against each other across multiple input sizes.
///
/// Generates statistical data for performance analysis and comparison using Criterion.
/// Optimized for fast benchmarking with reduced timing parameters.
///
/// # Parameters
/// * `c` - Criterion instance for benchmarking
/// * `group_name` - Benchmark group name (used for organization in reports)
/// * `algorithm1` - First algorithm definition (name + function)
/// * `algorithm2` - Second algorithm definition (name + function)
/// * `config` - Test configuration (sizes + input generation)
pub fn run_dual_algorithm_benchmark<'a, F1, F2, R, G>(
    c: &mut Criterion,
    group_name: &str,
    algorithm1: &Algorithm<'a, F1>,
    algorithm2: &Algorithm<'a, F2>,
    config: &TestConfig<'a, G>,
) where
    G: Fn(usize) -> String,
    F1: Fn(&str) -> R + Copy,
    F2: Fn(&str) -> R + Copy,
{
    let mut group = c.benchmark_group(group_name);

    for &size in config.sizes {
        let input = (config.generate_input)(size);

        // Benchmark first algorithm
        group.bench_with_input(
            BenchmarkId::new(algorithm1.name, size),
            &input,
            |b, input| b.iter(|| black_box((algorithm1.function)(black_box(input)))),
        );

        // Benchmark second algorithm
        group.bench_with_input(
            BenchmarkId::new(algorithm2.name, size),
            &input,
            |b, input| b.iter(|| black_box((algorithm2.function)(black_box(input)))),
        );
    }

    group.finish();
}

/// Processes benchmark results and generates visualization outputs.
///
/// Extracts timing data from Criterion JSON files, creates custom SVG plots,
/// and prints performance summaries. Uses Algorithm instances and TestConfig for
/// consistent signature with run_dual_algorithm_benchmark.
///
/// # Parameters
/// * `data_dir` - Directory where benchmark data is stored
/// * `group_name` - Benchmark group name used by Criterion
/// * `algorithm1` - Algorithm instance (name used for Criterion folder lookup)
/// * `algorithm2` - Algorithm instance (name used for Criterion folder lookup)
/// * `plot_config` - Display configuration for plot generation
/// * `test_config` - Test configuration containing sizes and input generation
///
/// # Examples
/// ```no_run
/// # use shared::benchmarking::{process_benchmark_results, PlotConfig, Algorithm, TestConfig};
/// # fn solve_naive(_: &str) -> i32 { 0 }
/// # fn solve_hashmap(_: &str) -> i32 { 0 }
/// # fn gen_input(_: usize) -> String { String::new() }
/// let algo1 = Algorithm { name: "naive", function: solve_naive };
/// let algo2 = Algorithm { name: "hashmap", function: solve_hashmap };
/// let test_config = TestConfig { sizes: &[1000, 5000], generate_input: gen_input };
/// let plot_config = PlotConfig {
///     filename: "naive_vs_hashmap.svg",
///     title: "Algorithm Performance Comparison",
///     algorithm1_name: "O(n²) Naive Algorithm",
///     algorithm2_name: "O(n) HashMap Solution",
/// };
/// process_benchmark_results("data", "criterion", &algo1, &algo2, &plot_config, &test_config);
/// ```
pub fn process_benchmark_results<F1, F2, R, G>(
    data_dir: &str,
    group_name: &str,
    algorithm1: &Algorithm<F1>,
    algorithm2: &Algorithm<F2>,
    plot_config: &PlotConfig,
    test_config: &TestConfig<G>,
) where
    F1: Fn(&str) -> R,
    F2: Fn(&str) -> R,
    G: Fn(usize) -> String,
{
    let plot_path = format!("{data_dir}/{}", plot_config.filename);

    match extract_criterion_results(
        data_dir,
        group_name,
        algorithm1.name,
        algorithm2.name,
        test_config.sizes,
    ) {
        Ok(results) => {
            // Generate custom plot co-located with data
            if let Err(e) = crate::plotting::create_dual_algorithm_plot(
                &plot_path,
                plot_config.title,
                plot_config.algorithm1_name,
                plot_config.algorithm2_name,
                &results,
            ) {
                eprintln!("Failed to create plot: {e}");
            } else {
                println!("✅ Benchmark complete!");
                println!("📊 View HTML reports: {data_dir}/{group_name}/report/index.html");
                println!("📈 Custom plot: {plot_path}");
            }

            // Print results summary
            print_benchmark_summary(&results);
        }
        Err(e) => {
            eprintln!("Failed to extract benchmark results: {e}");
        }
    }
}
/// Extracts timing data from Criterion JSON files.
///
/// Reads mean execution times from Criterion's estimates.json files and calculates
/// speedup ratios for performance comparison. Uses Criterion folder names to locate
/// the correct benchmark data directories.
///
/// # Parameters
/// * `base_path` - Base path where Criterion data is stored
/// * `group_name` - Benchmark group name used by Criterion
/// * `algo1_name` - Criterion folder name for first algorithm
/// * `algo2_name` - Criterion folder name for second algorithm  
/// * `sizes` - Array of input sizes that were tested
///
/// # Returns
/// Benchmark results as (size, algorithm1_time_ns, algorithm2_time_ns, speedup) tuples
///
/// # Errors
///
/// Returns an error if JSON files cannot be read or parsed.
///
/// # Examples
///
/// ```
/// # use shared::benchmarking::extract_criterion_results;
/// // Function extracts timing data from Criterion JSON files
/// // Example usage (requires actual benchmark data files):
/// // let results = extract_criterion_results(
/// //     "day01/data/criterion",
/// //     "performance_comparison",
/// //     "naive",        // Criterion folder name
/// //     "hashmap",      // Criterion folder name
/// //     &[1000, 5000, 10000]
/// // )?;
/// // results contains (size, naive_time_ns, hashmap_time_ns, speedup) tuples
/// ```
pub fn extract_criterion_results(
    base_path: &str,
    group_name: &str,
    algo1_name: &str,
    algo2_name: &str,
    sizes: &[usize],
) -> Result<BenchmarkResults> {
    let mut results = Vec::new();

    for &size in sizes {
        let algorithm1_path =
            format!("{base_path}/{group_name}/{algo1_name}/{size}/base/estimates.json");
        let algorithm2_path =
            format!("{base_path}/{group_name}/{algo2_name}/{size}/base/estimates.json");

        let algorithm1_time = read_criterion_estimate(&algorithm1_path)?;
        let algorithm2_time = read_criterion_estimate(&algorithm2_path)?;

        let speedup = algorithm2_time / algorithm1_time;
        results.push((size, algorithm1_time, algorithm2_time, speedup));
    }

    Ok(results)
}

/// Reads mean execution time from Criterion estimates.json file.
///
/// Parses the JSON structure generated by Criterion benchmarking to extract
/// the point estimate of mean execution time. This provides the primary timing
/// metric used for performance comparisons.
///
/// # Parameters
/// * `path` - Path to the Criterion estimates.json file
///
/// # Returns
/// Mean execution time in nanoseconds
///
/// # Errors
///
/// Returns an error if file cannot be read or JSON cannot be parsed.
fn read_criterion_estimate(path: &str) -> Result<f64> {
    let json_str = std::fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&json_str)?;

    // Extract mean estimate (in nanoseconds)
    let mean_ns = json["mean"]["point_estimate"]
        .as_f64()
        .context("Failed to parse mean estimate")?;

    Ok(mean_ns)
}

/// Prints benchmark results summary to stdout.
///
/// Displays formatted performance comparison with execution times in microseconds
/// and speedup ratios for each input size.
///
/// # Parameters
/// * `results` - Benchmark results to print
pub fn print_benchmark_summary(results: &BenchmarkResults) {
    for (size, algorithm1_time, algorithm2_time, speedup) in results {
        println!(
            "Size {}: Algorithm1 {:.2}μs, Algorithm2 {:.2}μs, Speedup {:.1}x",
            size,
            algorithm1_time / 1000.0,
            algorithm2_time / 1000.0,
            speedup
        );
    }
}
