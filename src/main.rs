// Logging Benchmark - Main Application
// Demonstrates usage of various Rust logging libraries

use chrono::Local;
use env_logger::Builder;
use log::{debug, error, info, trace, warn, LevelFilter};
use std::env;
use std::io::Write;

// Import additional logging implementations
use slog::o;
use slog::Drain;

fn main() {
    // Determine which logger to use from command-line args or env var
    let logger_type = env::args()
        .nth(1)
        .or_else(|| env::var("LOGGER").ok())
        .unwrap_or_else(|| "env_logger".to_string());

    // Setup the selected logger
    match logger_type.as_str() {
        "env_logger" => {
            setup_env_logger();
            println!("Using env_logger:");
            run_log_examples();
        }
        "log4rs" => {
            setup_log4rs();
            println!("Using log4rs:");
            run_log_examples();
        }
        "fern" => {
            setup_fern();
            println!("Using fern:");
            run_log_examples();
        }
        "ftlog" => {
            setup_ftlog();
            println!("Using ftlog:");
            run_log_examples();
        }
        "slog" => {
            let logger = setup_slog();
            println!("Using slog:");
            run_slog_examples(logger);
        }
        "tracing" => {
            setup_tracing();
            println!("Using tracing:");
            run_tracing_examples();
        }
        _ => {
            setup_env_logger();
            println!("Using default (env_logger):");
            run_log_examples();
        }
    }
}

// Setup functions for each logger

fn setup_env_logger() {
    Builder::new()
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

fn setup_log4rs() {
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

fn setup_fern() {
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

fn setup_ftlog() {
    let _ = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .try_init();
}

fn setup_slog() -> slog::Logger {
    use slog_async::Async;
    use slog_term::{FullFormat, TermDecorator};

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn setup_tracing() {
    use tracing_subscriber::fmt;

    fmt::init();
}

fn run_log_examples() {
    trace!("This is a trace message - typically not shown");
    debug!("This is a debug message - typically not shown");
    info!("This is an info message - should be visible");
    warn!("This is a warning message");
    error!("This is an error message");

    info!("The answer is {}", 42);

    info!(
        "User '{}' logged in from IP '{}' with status '{}'",
        "alice", "192.168.1.1", "success"
    );
}

fn run_slog_examples(logger: slog::Logger) {
    use slog::{debug, error, info, trace, warn};

    trace!(logger, "This is a trace message - typically not shown");
    debug!(logger, "This is a debug message - typically not shown");
    info!(logger, "This is an info message - should be visible");
    warn!(logger, "This is a warning message");
    error!(logger, "This is an error message");

    info!(logger, "The answer is {}", 42);

    info!(logger, "User logged in";
         "user" => "alice",
         "ip" => "192.168.1.1",
         "status" => "success"
    );
}

fn run_tracing_examples() {
    tracing::trace!("This is a trace message - typically not shown");
    tracing::debug!("This is a debug message - typically not shown");
    tracing::info!("This is an info message - should be visible");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");

    tracing::info!("The answer is {}", 42);

    let span = tracing::info_span!("user_login", user = "alice", ip = "192.168.1.1");
    let _guard = span.enter();
    tracing::info!(status = "success", "User logged in");
}
