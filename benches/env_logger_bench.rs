use criterion::{criterion_group, criterion_main, Criterion};
use log;
use std::io::{self, Write};
use std::sync::Once;

mod common;

// Initialize logger only once
static LOG_INIT: Once = Once::new();

// Standard log setup with env_logger
fn setup_env_logger() -> Result<(), io::Error> {
    LOG_INIT.call_once(|| {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                )
            })
            .try_init();
    });
    Ok(())
}

// Benchmark functions
fn bench_log(c: &mut Criterion) {
    setup_env_logger().unwrap();
    common::bench_message_sizes(c, "env_logger", |msg| {
        log::info!("{}", msg);
    });
}

// Benchmark different log levels
fn bench_log_levels(c: &mut Criterion) {
    setup_env_logger().unwrap();
    common::bench_log_levels(
        c,
        "env_logger_levels",
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
