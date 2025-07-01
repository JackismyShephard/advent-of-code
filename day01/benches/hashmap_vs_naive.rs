use std::hint::black_box;
use std::time::Instant;
use day01::{solve_part2, solve_part2_naive};
use plotters::prelude::*;

/// Simple Linear Congruential Generator for reproducible test data
/// Using parameters from Numerical Recipes
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    
    fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min + 1))
    }
}

/// Generate test input with reproducible pseudo-random data
/// Uses larger range (1-10000) to minimize hash collisions while maintaining diversity
fn generate_test_input(size: usize) -> String {
    let mut rng = SimpleRng::new(42); // Fixed seed for reproducibility
    let mut lines = Vec::with_capacity(size);
    
    for _ in 0..size {
        let left = rng.gen_range(1, 10000);
        let right = rng.gen_range(1, 10000);
        lines.push(format!("{left} {right}"));
    }
    
    lines.join("\n")
}

/// Benchmark a function with multiple iterations and warmup
fn benchmark_function<F>(f: F, input: &str, iterations: usize) -> f64 
where
    F: Fn(&str) -> Result<i32, anyhow::Error>,
{
    // Warmup - important for consistent timing
    for _ in 0..5 {
        let _ = black_box(f(black_box(input)));
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(f(black_box(input)));
    }
    let elapsed = start.elapsed();
    
    elapsed.as_nanos() as f64 / iterations as f64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Input sizes chosen to clearly demonstrate O(n) vs O(n²) scaling
    let sizes = vec![100, 500, 1000, 2000, 5000, 10000, 15000];
    let mut results = Vec::with_capacity(sizes.len());
    
    println!("Benchmarking O(n) hashmap vs O(n²) naive implementations...");
    println!("Size\tHashmap (μs)\tNaive (μs)\tSpeedup");
    println!("----\t------------\t----------\t-------");
    
    for &size in &sizes {
        let input = generate_test_input(size);
        
        // Use more iterations for smaller inputs, fewer for larger (to manage runtime)
        let iterations = if size <= 1000 { 50 } else if size <= 5000 { 20 } else { 10 };
        
        let hashmap_time = benchmark_function(solve_part2, &input, iterations);
        let naive_time = benchmark_function(solve_part2_naive, &input, iterations);
        
        let speedup = naive_time / hashmap_time;
        results.push((size, hashmap_time, naive_time, speedup));
        
        println!("{size}\t{:.2}\t\t{:.2}\t\t{speedup:.1}x", 
                 hashmap_time / 1000.0, 
                 naive_time / 1000.0);
    }
    
    create_performance_plot(&results)?;
    println!("\n✅ Performance comparison plot saved as 'performance_comparison.svg'");
    
    Ok(())
}

/// Create a logarithmic performance comparison plot using plotters
fn create_performance_plot(results: &[(usize, f64, f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("performance_comparison.svg", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let max_size = results.iter().map(|(size, _, _, _)| *size).max().unwrap();
    let min_time = results.iter()
        .flat_map(|(_, hashmap_time, naive_time, _)| [*hashmap_time, *naive_time])
        .fold(f64::INFINITY, f64::min);
    let max_time = results.iter()
        .flat_map(|(_, hashmap_time, naive_time, _)| [*hashmap_time, *naive_time])
        .fold(0.0, f64::max);
    
    // Use log scale for Y-axis to show both O(n) and O(n²) curves clearly
    let mut chart = ChartBuilder::on(&root)
        .caption("O(n) vs O(n²) Performance Comparison", ("sans-serif", 30))
        .margin(50)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(
            0f64..(max_size as f64 * 1.1), 
            (min_time * 0.5).log10()..(max_time * 2.0).log10()
        )?;
    
    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (nanoseconds)")
        .x_label_formatter(&|x| format!("{x:.0}"))
        .y_label_formatter(&|y| format!("{:.0}", 10f64.powf(*y) / 1000.0))
        .draw()?;
    
    // Plot hashmap (O(n)) performance using LineSeries (idiomatic)
    let hashmap_points: Vec<(f64, f64)> = results.iter()
        .map(|(size, hashmap_time, _, _)| (*size as f64, hashmap_time.log10()))
        .collect();
    
    chart
        .draw_series(LineSeries::new(hashmap_points.clone(), &BLUE))?
        .label("O(n) Hashmap")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));
    
    // Add data points for hashmap
    chart.draw_series(
        results.iter().map(|(size, hashmap_time, _, _)| 
            Circle::new((*size as f64, hashmap_time.log10()), 4, BLUE.filled())
        )
    )?;
    
    // Plot naive (O(n²)) performance using LineSeries
    let naive_points: Vec<(f64, f64)> = results.iter()
        .map(|(size, _, naive_time, _)| (*size as f64, naive_time.log10()))
        .collect();
    
    chart
        .draw_series(LineSeries::new(naive_points.clone(), &RED))?
        .label("O(n²) Naive")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
    
    // Add data points for naive
    chart.draw_series(
        results.iter().map(|(size, _, naive_time, _)| 
            Circle::new((*size as f64, naive_time.log10()), 4, RED.filled())
        )
    )?;
    
    // Add speedup annotations
    for (size, _, naive_time, speedup) in results {
        chart.draw_series(std::iter::once(Text::new(
            format!("{speedup:.1}x"), 
            (*size as f64, naive_time.log10() * 1.05), 
            ("sans-serif", 12)
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