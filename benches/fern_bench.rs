use criterion::{criterion_group, criterion_main, Criterion};
use log;
use std::sync::Once;

mod common;

// Initialize logger only once
static LOG_INIT: Once = Once::new();

// Fern setup
fn setup_fern() {
    LOG_INIT.call_once(|| {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    });
}

// Benchmark functions
fn bench_log(c: &mut Criterion) {
    setup_fern();
    common::bench_message_sizes(c, "fern", |msg| {
        log::info!("{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    setup_fern();
    common::bench_log_levels(
        c,
        "fern_levels",
        |msg| {
            log::trace!("{}", msg);
        },
        |msg| {
            log::debug!("{}", msg);
        },
        |msg| {
            log::info!("{}", msg);
        },
        |msg| {
            log::warn!("{}", msg);
        },
        |msg| {
            log::error!("{}", msg);
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
