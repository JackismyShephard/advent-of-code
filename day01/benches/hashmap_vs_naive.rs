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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        println!(
            "{size}\t{:.2}\t\t{:.2}\t\t{speedup:.1}x",
            hashmap_time / 1000.0,
            naive_time / 1000.0
        );
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
/// # Arguments
///
/// * `size: usize` - Number of input lines to generate
///
fn generate_test_input(size: usize) -> String {
    (0..size)
        .map(|i| format!("{} {}", (i % 9999) + 1, ((i * 7) % 9999) + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Benchmarks a function by measuring its median execution time.
///
/// Performs warmup iterations to stabilize CPU performance, then takes multiple
/// timing samples and returns the median to reduce noise from system interference.
///
/// # Arguments
///
/// * `f: F` - Function to benchmark that takes string input and returns a Result
/// * `input: &str` - Input string to pass to the function
/// * `samples: usize` - Number of timing samples to collect
///
fn benchmark_function<F>(f: F, input: &str, samples: usize) -> f64
where
    F: Fn(&str) -> Result<i32, anyhow::Error>,
{
    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        let _ = black_box(f(black_box(input)));
    }

    let mut times: Vec<f64> = (0..samples)
        .map(|_| {
            let start = Instant::now();
            let _ = black_box(f(black_box(input)));
            start.elapsed().as_nanos() as f64
        })
        .collect();

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    times[times.len() / 2]
}

/// Creates a performance comparison plot showing O(n) vs O(n²) algorithm scaling.
///
/// Generates an SVG chart with logarithmic y-axis showing execution times,
/// performance lines for both algorithms, and speedup factor labels.
///
/// # Arguments
///
/// * `results: &[(usize, f64, f64, f64)]` - Benchmark results as tuples of (size, hashmap_time, naive_time, speedup)
///
/// # Errors
///
/// Returns `Err` if chart creation fails.
fn create_performance_plot(
    results: &[(usize, f64, f64, f64)],
) -> Result<(), Box<dyn std::error::Error>> {
    let (root, mut chart) = setup_chart(results)?;

    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (nanoseconds)")
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
/// # Arguments
///
/// * `results: &[(usize, f64, f64, f64)]` - Benchmark data used to determine axis ranges
///
/// # Errors
///
/// Returns `Err` if chart setup fails.
fn setup_chart(
    results: &[(usize, f64, f64, f64)],
) -> Result<
    (
        DrawingArea<SVGBackend, plotters::coord::Shift>,
        PlotChart<'_>,
    ),
    Box<dyn std::error::Error>,
> {
    let root = SVGBackend::new("performance_comparison.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = results.iter().map(|(size, _, _, _)| *size).max().unwrap();
    let times: Vec<f64> = results.iter().flat_map(|(_, h, n, _)| [*h, *n]).collect();
    let (min_time, max_time) = (
        times.iter().copied().fold(f64::INFINITY, f64::min),
        times.iter().copied().fold(0.0, f64::max),
    );

    let chart = ChartBuilder::on(&root)
        .caption("Performance Comparison", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(60)
        .y_label_area_size(80)
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
/// # Arguments
///
/// * `chart: &mut PlotChart<'_>` - Mutable reference to the chart context
/// * `results: &[(usize, f64, f64, f64)]` - Benchmark results containing timing data
///
/// # Errors
///
/// Returns `Err` if plotting fails.
fn plot_performance_lines(
    chart: &mut PlotChart<'_>,
    results: &[(usize, f64, f64, f64)],
) -> Result<(), Box<dyn std::error::Error>> {
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
/// # Arguments
///
/// * `chart: &mut PlotChart<'_>` - Mutable reference to the chart context
/// * `results: &[(usize, f64, f64, f64)]` - Benchmark results containing speedup factors
///
/// # Errors
///
/// Returns `Err` if label rendering fails.
fn add_speedup_labels(
    chart: &mut PlotChart<'_>,
    results: &[(usize, f64, f64, f64)],
) -> Result<(), Box<dyn std::error::Error>> {
    for (size, _, naive_time, speedup) in results {
        chart.draw_series(std::iter::once(Text::new(
            format!("{speedup:.1}x"),
            (*size as f64, naive_time.log10() * 1.05),
            ("sans-serif", 12),
        )))?;
    }
    Ok(())
}

/// Draws a performance line with circular markers and legend entry.
///
/// Helper function that creates both the line series and point markers
/// for a single algorithm's performance data.
///
/// # Arguments
///
/// * `chart: &mut PlotChart<'a>` - Mutable reference to the chart context
/// * `points: &[(f64, f64)]` - Coordinate pairs for the line
/// * `color: &'a RGBColor` - Line and marker color
/// * `label: &str` - Legend label for this line
///
/// # Errors
///
/// Returns `Err` if drawing fails.
fn draw_line_with_points<'a>(
    chart: &mut PlotChart<'a>,
    points: &[(f64, f64)],
    color: &'a RGBColor,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
