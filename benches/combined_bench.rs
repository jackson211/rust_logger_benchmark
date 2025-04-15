use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use log;
use log_benchmark::logger_setup::{
    // Import benchmark setups
    setup_env_logger_bench,
    setup_fern_bench,
    setup_log4rs_bench,
    // Import standard setup for slog
    setup_slog,
    setup_slog_bench,
    setup_tracing_bench,
};
use slog;
use tracing;

// Renamed original function
fn benchmark_performance_comparison(c: &mut Criterion) {
    // --- Setup Loggers (Benchmark Configs) ---
    let (slog_logger_bench, _slog_metrics) = setup_slog_bench();
    let _fern_metrics = setup_fern_bench();
    let _log4rs_metrics = setup_log4rs_bench();
    let _tracing_metrics = setup_tracing_bench();
    let _env_logger_metrics = setup_env_logger_bench();

    // --- Message Size Comparison (Benchmark Configs) ---
    let mut group = c.benchmark_group("Log Message Size Comparison (Bench Configs)");
    for size in [10, 100, 1000].iter() {
        let msg = "X".repeat(*size);
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("slog", size), &msg, |b, m| {
            b.iter(|| slog::info!(slog_logger_bench, "{}", m))
        });
        group.bench_with_input(BenchmarkId::new("fern", size), &msg, |b, m| {
            b.iter(|| log::info!("{}", m))
        });
        group.bench_with_input(BenchmarkId::new("log4rs", size), &msg, |b, m| {
            b.iter(|| log::info!("{}", m))
        });
        group.bench_with_input(BenchmarkId::new("tracing", size), &msg, |b, m| {
            b.iter(|| tracing::info!("{}", m))
        });
        group.bench_with_input(BenchmarkId::new("env_logger", size), &msg, |b, m| {
            b.iter(|| log::info!("{}", m))
        });
    }
    group.finish();

    // --- Log Level Comparison (INFO Level - Bench Configs) ---
    let mut group = c.benchmark_group("Log Level Comparison (Info Level - Bench Configs)");
    let msg = "Info message for level comparison";
    group.throughput(Throughput::Elements(1));

    group.bench_function("slog", |b| {
        b.iter(|| slog::info!(slog_logger_bench, "{}", msg))
    });
    group.bench_function("fern", |b| b.iter(|| log::info!("{}", msg)));
    group.bench_function("log4rs", |b| b.iter(|| log::info!("{}", msg)));
    group.bench_function("tracing", |b| b.iter(|| tracing::info!("{}", msg)));
    group.bench_function("env_logger", |b| b.iter(|| log::info!("{}", msg)));
    group.finish();
}

// New function to compare slog standard vs benchmark setup
fn benchmark_slog_configs(c: &mut Criterion) {
    let msg = "Simple info message";
    let mut group = c.benchmark_group("Slog Config Comparison (INFO)");
    group.throughput(Throughput::Elements(1));

    // Benchmark Standard Slog Setup
    let slog_logger_std = setup_slog();
    group.bench_function("slog_standard", |b| {
        b.iter(|| slog::info!(slog_logger_std, "{}", msg))
    });

    // Benchmark Benchmark-Optimized Slog Setup
    let (slog_logger_bench, _metrics) = setup_slog_bench();
    group.bench_function("slog_bench", |b| {
        b.iter(|| slog::info!(slog_logger_bench, "{}", msg))
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    // Add both functions to targets
    targets = benchmark_performance_comparison, benchmark_slog_configs
);
criterion_main!(benches);
