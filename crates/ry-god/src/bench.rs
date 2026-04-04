//! Benchmark - Demostrar eficiencia
//!
//! ry-god vs el mundo: low-end primero.

use std::time::Instant;

/// Resultado de benchmark
pub struct BenchmarkResult {
    pub iterations: u32,
    pub total_ms: u128,
    pub avg_ms: f64,
    pub min_ms: u128,
    pub max_ms: u128,
}

/// Benchmark simple sin dependencias
pub struct Benchmark;

impl Benchmark {
    pub fn run(iterations: u32) -> BenchmarkResult {
        let mut times = Vec::with_capacity(iterations as usize);

        for _ in 0..iterations {
            let start = Instant::now();
            // Simulación de parsing: string operations básicas
            let _test = "dark.slot x = 400".to_string().len();
            times.push(start.elapsed().as_micros());
        }

        let total: u128 = times.iter().sum();
        let min = *times.iter().min().unwrap_or(&0);
        let max = *times.iter().max().unwrap_or(&0);
        let avg = total as f64 / iterations as f64;

        BenchmarkResult {
            iterations,
            total_ms: total / 1000,
            avg_ms: avg / 1000.0,
            min_ms: min / 1000,
            max_ms: max / 1000,
        }
    }

    /// Imprimir resultados
    pub fn print(result: &BenchmarkResult) {
        println!("\n📊 BENCHMARK ry-god");
        println!("{}", "=".repeat(40));
        println!("Iteraciones: {}", result.iterations);
        println!("Total:       {}ms", result.total_ms);
        println!("Promedio:    {:.3}ms", result.avg_ms);
        println!("Mínimo:      {}ms", result.min_ms);
        println!("Máximo:      {}ms", result.max_ms);
        println!("{}", "=".repeat(40));
    }
}
