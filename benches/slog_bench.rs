use criterion::{Criterion, criterion_group, criterion_main};
use slog::{Drain, Logger, o};
use std::sync::Mutex;

mod common;

// Setup for async logger (primary implementation)
fn setup_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();

    // Configure with the standard timestamp format
    let drain = slog_term::FullFormat::new(decorator)
        // slog's timestamp format is hardcoded, but it uses the same format as our standard
        // %Y-%m-%d %H:%M:%S%.3f by default when use_local_timestamp() is set
        .use_local_timestamp()
        .build()
        .fuse();

    // Use a more moderate buffer size to prevent memory issues
    // and use a non-blocking strategy that drops logs if the buffer is full
    let async_drain = slog_async::Async::new(drain)
        .chan_size(8192) // More moderate buffer size
        .overflow_strategy(slog_async::OverflowStrategy::Drop) // Non-blocking strategy
        .build();

    // Create the logger
    slog::Logger::root(async_drain.fuse(), o!())
}

// Alternative synchronous logger setup for comparison
// This avoids async issues entirely but may be slower
fn setup_sync_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();

    let drain = slog_term::FullFormat::new(decorator)
        .use_local_timestamp()
        .build()
        .fuse();

    // Use a mutex to make the drain thread-safe without async
    let drain = Mutex::new(drain).fuse();
    slog::Logger::root(drain, o!())
}

// Main benchmark function
fn bench_log(c: &mut Criterion) {
    // Try using the sync logger as a fallback if async is problematic
    // let log = setup_sync_logger();
    let log = setup_logger();
    common::bench_message_sizes(c, "slog", |msg| {
        slog::info!(log, "{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    // Try using the sync logger as a fallback if async is problematic
    // let log = setup_sync_logger();
    let log = setup_logger();
    common::bench_log_levels(
        c,
        "slog_levels",
        |msg| {
            slog::trace!(log, "{}", msg);
        },
        |msg| {
            slog::debug!(log, "{}", msg);
        },
        |msg| {
            slog::info!(log, "{}", msg);
        },
        |msg| {
            slog::warn!(log, "{}", msg);
        },
        |msg| {
            slog::error!(log, "{}", msg);
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
