use criterion::{Criterion, criterion_group, criterion_main};
use log;
use std::sync::Once;

mod common;

// Initialize logger only once
static LOG_INIT: Once = Once::new();

// Log4rs setup
fn setup_log4rs() {
    LOG_INIT.call_once(|| {
        // Set up a pattern encoder for formatting logs
        let pattern =
            log4rs::encode::pattern::PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)} {l} {m}{n}");

        // Create an appender that writes to stdout with the pattern encoder
        let stdout = log4rs::append::console::ConsoleAppender::builder()
            .encoder(Box::new(pattern))
            .build();

        // Build a config
        let config = log4rs::config::Config::builder()
            .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                log4rs::config::Root::builder()
                    .appender("stdout")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        // Initialize the logger with the config
        let _ = log4rs::init_config(config).unwrap();
    });
}

// Benchmark functions
fn bench_log(c: &mut Criterion) {
    setup_log4rs();
    common::bench_message_sizes(c, "log4rs", |msg| {
        log::info!("{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    setup_log4rs();
    common::bench_log_levels(
        c,
        "log4rs_levels",
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
