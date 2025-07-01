use day01::{solve_part2, solve_part2_naive};
use plotters::prelude::*;
use std::hint::black_box;
use std::time::Instant;

fn generate_test_input(size: usize) -> String {
    (0..size)
        .map(|i| format!("{} {}", (i % 9999) + 1, ((i * 7) % 9999) + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn benchmark_function<F>(f: F, input: &str, samples: usize) -> f64
where
    F: Fn(&str) -> Result<i32, anyhow::Error>,
{
    // Warmup
    for _ in 0..3 {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sizes = [500, 1000, 2000, 5000, 8000, 12000];
    let mut results = Vec::new();

    println!("Size\tHashmap (μs)\tNaive (μs)\tSpeedup");
    println!("----\t------------\t----------\t-------");

    for &size in &sizes {
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

/// Create performance comparison plot
fn create_performance_plot(
    results: &[(usize, f64, f64, f64)],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("performance_comparison.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = results.iter().map(|(size, _, _, _)| *size).max().unwrap();
    let times: Vec<f64> = results.iter().flat_map(|(_, h, n, _)| [*h, *n]).collect();
    let (min_time, max_time) = (
        times.iter().copied().fold(f64::INFINITY, f64::min),
        times.iter().copied().fold(0.0, f64::max),
    );

    let mut chart = ChartBuilder::on(&root)
        .caption("O(n) vs O(n²) Performance Comparison", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(
            0f64..(max_size as f64 * 1.1),
            (min_time * 0.5).log10()..(max_time * 2.0).log10(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (nanoseconds)")
        .x_label_formatter(&|x| format!("{x:.0}"))
        .y_label_formatter(&|y| format!("{:.0}", 10f64.powf(*y) / 1000.0))
        .draw()?;

    // Plot both lines and points
    let hashmap_points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, h, _, _)| (*size as f64, h.log10()))
        .collect();
    let naive_points: Vec<(f64, f64)> = results
        .iter()
        .map(|(size, _, n, _)| (*size as f64, n.log10()))
        .collect();

    chart
        .draw_series(LineSeries::new(hashmap_points.clone(), &BLUE))?
        .label("O(n) Hashmap")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));
    chart.draw_series(
        hashmap_points
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 4, BLUE.filled())),
    )?;

    chart
        .draw_series(LineSeries::new(naive_points.clone(), &RED))?
        .label("O(n²) Naive")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
    chart.draw_series(
        naive_points
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 4, RED.filled())),
    )?;

    for (size, _, naive_time, speedup) in results {
        chart.draw_series(std::iter::once(Text::new(
            format!("{speedup:.1}x"),
            (*size as f64, naive_time.log10() * 1.05),
            ("sans-serif", 12),
        )))?;
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
