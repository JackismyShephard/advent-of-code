use anyhow::Result;
use day01::{solve_part2, solve_part2_naive};
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use std::hint::black_box;
use std::time::Instant;

const SIZES: [usize; 6] = [500, 1000, 2000, 5000, 8000, 12000];
const WARMUP_ITERATIONS: usize = 3;
const CIRCLE_RADIUS: i32 = 4;
const LEGEND_LINE_LENGTH: i32 = 10;

type PlotChart<'a> = ChartContext<'a, SVGBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

fn main() -> Result<()> {
    let mut results = Vec::new();

    println!("Size\tHashmap (μs)\tNaive (μs)\tSpeedup");
    println!("----\t------------\t----------\t-------");

    for &size in &SIZES {
        let input = generate_test_input(size);
        let samples = if size <= 2000 { 10 } else { 5 };

        let hashmap_time = benchmark_function(solve_part2, &input, samples);
        let naive_time = benchmark_function(solve_part2_naive, &input, samples);
        let speedup = naive_time / hashmap_time;

        results.push((size, hashmap_time, naive_time, speedup));

        let hashmap_us = hashmap_time / 1000.0;
        let naive_us = naive_time / 1000.0;

        println!("{size}\t{hashmap_us:.2}\t\t{naive_us:.2}\t\t{speedup:.1}x");
    }

    create_performance_plot(&results)?;
    println!("\n✅ Performance plot saved as 'performance_comparison.svg'");

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
fn benchmark_function<F>(algorithm_fn: F, test_input: &str, sample_count: usize) -> f64
where
    F: Fn(&str) -> Result<i32, anyhow::Error>,
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

/// Creates a performance comparison plot showing O(n) vs O(n²) algorithm scaling.
///
/// Generates an SVG chart with logarithmic y-axis showing execution times,
/// performance lines for both algorithms, and speedup factor labels.
///
/// # Parameters
/// * `results` - Benchmark data as tuples of (input_size, hashmap_time_ns, naive_time_ns, speedup_factor)
///
/// # Returns
/// `Ok(())` if chart creation succeeds
///
/// # Errors
///
/// Returns `Err` if chart creation fails.
fn create_performance_plot(results: &[(usize, f64, f64, f64)]) -> Result<()> {
    let (root, mut chart) = setup_chart(results)?;

    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (microseconds)")
        .x_label_formatter(&|x| format!("{x:.0}"))
        .y_label_formatter(&|y| format!("{:.0}", 10f64.powf(*y) / 1000.0))
        .draw()?;

    plot_performance_lines(&mut chart, results)?;
    add_speedup_labels(&mut chart, results)?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// Sets up the chart layout and coordinate system for performance plotting.
///
/// Creates the SVG backend, determines appropriate axis ranges from data,
/// and configures the chart with logarithmic y-axis scaling.
///
/// # Parameters
/// * `results` - Benchmark data used to determine chart axis ranges and scaling
///
/// # Returns
/// Tuple of (drawing_area, configured_chart) ready for data plotting
///
/// # Errors
///
/// Returns `Err` if chart setup fails.
fn setup_chart(
    results: &[(usize, f64, f64, f64)],
) -> Result<(
    DrawingArea<SVGBackend, plotters::coord::Shift>,
    PlotChart<'_>,
)> {
    let root = SVGBackend::new("performance_comparison.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = results
        .iter()
        .map(|(size, _, _, _)| *size)
        .max()
        .ok_or_else(|| anyhow::anyhow!("No benchmark results to plot"))?;
    let times: Vec<f64> = results.iter().flat_map(|(_, h, n, _)| [*h, *n]).collect();
    let (min_time, max_time) = (
        times.iter().copied().fold(f64::INFINITY, f64::min),
        times.iter().copied().fold(0.0, f64::max),
    );

    let chart = ChartBuilder::on(&root)
        .caption("Performance Comparison", ("sans-serif", 24))
        .margin(50)
        .margin_top(80)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0f64..(max_size as f64 * 1.1),
            (min_time * 0.5).log10()..(max_time * 2.0).log10(),
        )?;

    Ok((root, chart))
}
/// Plots the performance lines for both hashmap and naive algorithms.
///
/// Extracts timing data, applies logarithmic transformation, and draws
/// lines with points for both algorithm implementations.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing operations
/// * `results` - Benchmark data as tuples of (input_size, hashmap_time_ns, naive_time_ns, speedup_factor)
///
/// # Returns
/// `Ok(())` if plotting succeeds
///
/// # Errors
///
/// Returns `Err` if plotting fails.
fn plot_performance_lines(
    chart: &mut PlotChart<'_>,
    results: &[(usize, f64, f64, f64)],
) -> Result<()> {
    let hashmap_points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, h, _, _)| (*size as f64, h.log10()))
        .collect();
    let naive_points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, _, n, _)| (*size as f64, n.log10()))
        .collect();

    draw_line_with_points(chart, &hashmap_points, &BLUE, "O(n) Hashmap")?;
    draw_line_with_points(chart, &naive_points, &RED, "O(n²) Naive")?;

    Ok(())
}

/// Adds speedup factor labels above the naive algorithm performance line.
///
/// Places text annotations showing the performance improvement factor
/// at each data point for easy interpretation of results.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing text labels
/// * `results` - Benchmark data as tuples of (input_size, hashmap_time_ns, naive_time_ns, speedup_factor)
///
/// # Returns
/// `Ok(())` if label rendering succeeds
///
/// # Errors
///
/// Returns `Err` if label rendering fails.
fn add_speedup_labels(chart: &mut PlotChart<'_>, results: &[(usize, f64, f64, f64)]) -> Result<()> {
    let labels: Vec<_> = results
        .iter()
        .map(|(size, _, naive_time, speedup)| {
            Text::new(
                format!("{speedup:.1}x"),
                (*size as f64, naive_time.log10() * 1.05),
                ("sans-serif", 12),
            )
        })
        .collect();

    chart.draw_series(labels)?;
    Ok(())
}

/// Draws a performance line with circular markers and legend entry.
///
/// Helper function that creates both the line series and point markers
/// for a single algorithm's performance data.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing operations
/// * `points` - Array of (x, y) coordinates representing algorithm performance data
/// * `color` - Reference to the RGB color for drawing the line and markers
/// * `label` - Text label for the legend entry describing this algorithm
///
/// # Returns
/// `Ok(())` if drawing succeeds
///
/// # Errors
///
/// Returns `Err` if drawing fails.
fn draw_line_with_points<'a>(
    chart: &mut PlotChart<'a>,
    points: &[(f64, f64)],
    color: &'a RGBColor,
    label: &str,
) -> Result<()> {
    chart
        .draw_series(LineSeries::new(points.iter().copied(), color))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + LEGEND_LINE_LENGTH, y)], *color));
    chart.draw_series(
        points
            .iter()
            .map(|&(x, y)| Circle::new((x, y), CIRCLE_RADIUS, color.filled())),
    )?;
    Ok(())
}
