use criterion::{criterion_group, criterion_main, Criterion};
use std::io;
use std::sync::Once;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::fmt;

mod common;

// Initialize logger only once
static LOG_INIT: Once = Once::new();

// Standard log setup with tracing
fn setup_tracing() -> Result<(), io::Error> {
    LOG_INIT.call_once(|| {
        // Note: tracing's default format may differ slightly from other loggers
        // This is acceptable for benchmarking purposes
        fmt::init();
    });
    Ok(())
}

// Benchmark functions
fn bench_log(c: &mut Criterion) {
    setup_tracing().unwrap();
    common::bench_message_sizes(c, "tracing", |msg| {
        info!("{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    setup_tracing().unwrap();
    common::bench_log_levels(
        c,
        "tracing_levels",
        |msg| {
            trace!("{}", msg);
        },
        |msg| {
            debug!("{}", msg);
        },
        |msg| {
            info!("{}", msg);
        },
        |msg| {
            warn!("{}", msg);
        },
        |msg| {
            error!("{}", msg);
        },
    );
}

// Create benchmark group
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_log, bench_log_levels
);
criterion_main!(benches);
