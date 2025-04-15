// src/logger_setup/standard_setups.rs
use chrono::Local;
use log::LevelFilter;
use slog::{o, Drain, Logger};
use std::io::Write;

// ===== Standard Logger Setup Functions (for normal usage) =====

// Set up env_logger for normal usage
pub fn setup_env_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

// Set up log4rs for normal usage
pub fn setup_log4rs() {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] - {t}: {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    let _ = log4rs::init_config(config);
}

// Set up fern for normal usage
pub fn setup_fern() {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {}: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .apply();
}

// Set up ftlog for normal usage
pub fn setup_ftlog() {
    let _ = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .root(ftlog::appender::ChainAppenders::new(vec![Box::new(
            std::io::stdout(),
        )]))
        .try_init();

    // Need to access the static from the parent or common module
    // super::common::NEEDS_FTLOG_CLOSE.store(true, super::common::Ordering::SeqCst);
    // OR pass the static flag ref if LoggerGuard is elsewhere
    // For now, commenting out as statics are in common.rs
    // crate::logger_setup::common::NEEDS_FTLOG_CLOSE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// Set up slog for normal usage
pub fn setup_slog() -> Logger {
    use slog_async::Async;
    use slog_term::{FullFormat, TermDecorator};

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    Logger::root(drain, o!())
}

// Set up tracing for normal usage
pub fn setup_tracing() {
    use tracing_subscriber::fmt;
    fmt::init();
}
