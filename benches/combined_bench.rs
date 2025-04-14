use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use log;
use log_benchmark::logger_setup::{
    setup_env_logger_bench, setup_fern_bench, setup_ftlog_bench, setup_log4rs_bench,
    setup_slog_bench, setup_tracing_bench,
};
use slog;
use tracing;

fn benchmark_loggers(c: &mut Criterion) {
    // --- Setup Loggers ---
    let (slog_logger, _slog_metrics) = setup_slog_bench();
    let _fern_metrics = setup_fern_bench();
    let _ftlog_metrics = setup_ftlog_bench();
    let _log4rs_metrics = setup_log4rs_bench();
    let _tracing_metrics = setup_tracing_bench();
    let _env_logger_metrics = setup_env_logger_bench();

    // --- Message Size Comparison ---
    let mut group = c.benchmark_group("Log Message Size Comparison");
    for size in [10, 100, 1000].iter() {
        let msg = "X".repeat(*size);
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("slog", size), &msg, |b, m| {
            b.iter(|| slog::info!(slog_logger, "{}", m))
        });
        group.bench_with_input(BenchmarkId::new("fern", size), &msg, |b, m| {
            b.iter(|| log::info!("{}", m))
        });
        group.bench_with_input(BenchmarkId::new("ftlog", size), &msg, |b, m| {
            b.iter(|| ftlog::info!("{}", m))
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

    // --- Log Level Comparison (INFO Level) ---
    let mut group = c.benchmark_group("Log Level Comparison (Info Level)");
    let msg = "Info message for level comparison";
    group.throughput(Throughput::Elements(1));

    group.bench_function("slog", |b| b.iter(|| slog::info!(slog_logger, "{}", msg)));
    group.bench_function("fern", |b| b.iter(|| log::info!("{}", msg)));
    group.bench_function("ftlog", |b| b.iter(|| ftlog::info!("{}", msg)));
    group.bench_function("log4rs", |b| b.iter(|| log::info!("{}", msg)));
    group.bench_function("tracing", |b| b.iter(|| tracing::info!("{}", msg)));
    group.bench_function("env_logger", |b| b.iter(|| log::info!("{}", msg)));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = benchmark_loggers
);
criterion_main!(benches);
