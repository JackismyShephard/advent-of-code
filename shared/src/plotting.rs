//! Shared plotting utilities for benchmark visualization.
//!
//! This module provides simple plotting functionality for creating
//! performance comparison charts across different days of Advent of Code.

use anyhow::Result;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

const CIRCLE_RADIUS: i32 = 4;
const LEGEND_LINE_LENGTH: i32 = 10;
const CHART_WIDTH: u32 = 800;
const CHART_HEIGHT: u32 = 600;

type PlotChart<'a> = ChartContext<'a, SVGBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

/// Creates a dual-algorithm performance comparison plot.
///
/// Generates an SVG chart comparing two algorithms with fixed styling:
/// - 800x600 dimensions
/// - Blue line for first algorithm, red line for second
/// - Logarithmic y-axis showing execution times
/// - Speedup factor labels
/// - Standard "Input Size (n)" / "Time (microseconds)" axes
///
/// # Parameters
/// * `day` - Day number for filename and title generation
/// * `algo1_name` - Name of the first algorithm
/// * `algo2_name` - Name of the second algorithm
/// * `results` - Benchmark data as (input_size, time1_ns, time2_ns, speedup) tuples
///
/// # Returns
/// `Ok(())` if chart creation succeeds
///
/// # Errors
///
/// Returns `Err` if chart creation fails.
pub fn create_dual_algorithm_plot(
    day: u8,
    algo1_name: &str,
    algo2_name: &str,
    results: &[(usize, f64, f64, f64)],
) -> Result<()> {
    let filename = format!("day{day:02}_performance_comparison.svg");
    let title = format!("Day {day}: {algo1_name} vs {algo2_name} Algorithm Performance");

    let (root, mut chart) = setup_performance_chart(&filename, &title, results)?;

    // Configure mesh for performance benchmark charts
    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (microseconds)")
        .x_label_formatter(&|x| format!("{x:.0}"))
        .y_label_formatter(&|y| format!("{:.0}", 10f64.powf(*y) / 1000.0))
        .draw()?;

    // Plot both algorithms
    plot_performance_line(&mut chart, results, 0, &BLUE, algo1_name)?;
    plot_performance_line(&mut chart, results, 1, &RED, algo2_name)?;

    // Add speedup labels above the second algorithm line
    add_speedup_labels(&mut chart, results)?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("✅ Performance plot saved as '{filename}'");
    Ok(())
}

/// Sets up the chart layout and coordinate system for performance benchmarks.
///
/// Creates the SVG backend, determines appropriate axis ranges from timing data,
/// and builds the chart with logarithmic y-axis scaling for performance visualization.
///
/// # Parameters
/// * `filename` - Output SVG filename
/// * `title` - Chart title
/// * `results` - Benchmark data used to determine axis ranges
///
/// # Returns
/// Tuple of (drawing_area, configured_chart) ready for mesh configuration and data plotting
///
/// # Errors
///
/// Returns `Err` if chart setup fails.
fn setup_performance_chart<'a>(
    filename: &'a str,
    title: &'a str,
    results: &[(usize, f64, f64, f64)],
) -> Result<(
    DrawingArea<SVGBackend<'a>, plotters::coord::Shift>,
    PlotChart<'a>,
)> {
    let root = SVGBackend::new(filename, (CHART_WIDTH, CHART_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = results
        .iter()
        .map(|(size, _, _, _)| *size)
        .max()
        .ok_or_else(|| anyhow::anyhow!("No data points to plot"))?;
    let times: Vec<f64> = results
        .iter()
        .flat_map(|(_, t1, t2, _)| [*t1, *t2])
        .collect();
    let (min_time, max_time) = (
        times.iter().copied().fold(f64::INFINITY, f64::min),
        times.iter().copied().fold(0.0, f64::max),
    );

    let chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
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

/// Plots a single algorithm's performance line.
///
/// Extracts timing data for one algorithm, applies logarithmic transformation,
/// and draws a line with points for that algorithm's performance.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing operations
/// * `results` - Benchmark data as tuples of (input_size, time1_ns, time2_ns, speedup_factor)
/// * `time_index` - Which time column to use (0 for first algorithm, 1 for second)
/// * `color` - Color for the line and markers
/// * `label` - Label for the legend entry
///
/// # Returns
/// `Ok(())` if plotting succeeds
///
/// # Errors
///
/// Returns `Err` if plotting fails.
fn plot_performance_line<'a>(
    chart: &mut PlotChart<'a>,
    results: &[(usize, f64, f64, f64)],
    time_index: usize,
    color: &'a RGBColor,
    label: &str,
) -> Result<()> {
    let points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, time1, time2, _)| {
            let time = if time_index == 0 { *time1 } else { *time2 };
            (*size as f64, time.log10())
        })
        .collect();

    draw_line_with_points(chart, &points, color, label)
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

/// Adds speedup factor labels above the second algorithm performance line.
///
/// Places text annotations showing the performance improvement factor
/// at each data point for easy interpretation of results.
///
/// # Parameters
/// * `chart` - Mutable reference to the chart context for drawing text labels
/// * `results` - Benchmark data as tuples of (input_size, time1_ns, time2_ns, speedup_factor)
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
        .map(|(size, _, time2, speedup)| {
            Text::new(
                format!("{speedup:.1}x"),
                (*size as f64, time2.log10() * 1.05),
                ("sans-serif", 12),
            )
        })
        .collect();

    chart.draw_series(labels)?;
    Ok(())
}
