use criterion::{Criterion, criterion_group, criterion_main};
use ftlog::LevelFilter as FtLevelFilter;
use std::sync::Once;

mod common;

// Initialize logger only once
static LOG_INIT: Once = Once::new();

// Setup ftlog
fn setup_ftlog() -> Result<(), ()> {
    LOG_INIT.call_once(|| {
        // Note: FTLog doesn't provide timestamp formatting options
        // so we're accepting that it uses a different format
        let _ = ftlog::builder()
            .max_log_level(FtLevelFilter::Info)
            .try_init()
            .unwrap();
    });
    Ok(())
}

// Benchmark functions
fn bench_log(c: &mut Criterion) {
    setup_ftlog().unwrap();
    common::bench_message_sizes(c, "ftlog", |msg| {
        ftlog::info!("{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    setup_ftlog().unwrap();
    common::bench_log_levels(
        c,
        "ftlog_levels",
        |msg| {
            ftlog::trace!("{}", msg);
        },
        |msg| {
            ftlog::debug!("{}", msg);
        },
        |msg| {
            ftlog::info!("{}", msg);
        },
        |msg| {
            ftlog::warn!("{}", msg);
        },
        |msg| {
            ftlog::error!("{}", msg);
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
