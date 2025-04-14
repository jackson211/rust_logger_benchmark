use crate::metrics::MessageStats;
use chrono::{Local, Utc};
use fern::Dispatch;
use log::{LevelFilter, Log};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{
    config::{Appender, Config, Root},
    init_config,
};
use slog::{o, Drain, Logger, Never, KV};
use slog_async::Async;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use tracing::subscriber::set_global_default;
use tracing::Level as TracingLevel;
use tracing_subscriber::fmt;

// Metrics tracking sink that counts messages and bytes
// Used for benchmarking
#[derive(Debug, Default, Clone)]
pub struct MetricsSink {
    count: Arc<Mutex<usize>>,
    bytes: Arc<Mutex<usize>>,
}

impl MetricsSink {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
            bytes: Arc::new(Mutex::new(0)),
        }
    }

    pub fn record(&self, bytes: usize) {
        let mut count = self.count.lock().unwrap();
        let mut total_bytes = self.bytes.lock().unwrap();
        *count += 1;
        *total_bytes += bytes;
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        let count = *self.count.lock().unwrap();
        let bytes = *self.bytes.lock().unwrap();
        (count, bytes)
    }

    // Reset metrics for a new benchmark run
    pub fn reset(&self) {
        let mut count = self.count.lock().unwrap();
        let mut total_bytes = self.bytes.lock().unwrap();
        *count = 0;
        *total_bytes = 0;
    }
}

// Wrapper type for Arc<MetricsSink> to implement Write
pub struct MetricsSinkWriter(pub Arc<MetricsSink>);

impl Write for MetricsSinkWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = buf.len();
        self.0.record(len);
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Custom Drain for slog that discards output but tracks metrics
pub struct MetricsDrain {
    pub metrics: Arc<MetricsSink>,
}

impl slog::Drain for MetricsDrain {
    type Ok = ();
    type Err = Never;

    fn log(&self, record: &slog::Record, _: &slog::OwnedKVList) -> Result<Self::Ok, Self::Err> {
        // Format a log message similar to what other loggers would produce
        let formatted = format!(
            "{} [{}] - {}: {}",
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.module(),
            record.msg()
        );
        self.metrics.record(formatted.len());
        Ok(())
    }
}

// Global flag to track if we need to close the async drain
static NEEDS_ASYNC_CLOSE: AtomicBool = AtomicBool::new(false);
static ASYNC_DRAIN: std::sync::OnceLock<Arc<slog_async::Async>> = std::sync::OnceLock::new();
// Global flag to track if we need to close ftlog
static NEEDS_FTLOG_CLOSE: AtomicBool = AtomicBool::new(false);

// Global counters for metrics
static MESSAGE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTE_COUNT: AtomicUsize = AtomicUsize::new(0);

// Helper functions for metrics
pub fn reset_metrics() {
    MESSAGE_COUNT.store(0, Ordering::SeqCst);
    BYTE_COUNT.store(0, Ordering::SeqCst);
}

pub fn get_metrics() -> (usize, usize) {
    let count = MESSAGE_COUNT.load(Ordering::SeqCst);
    let bytes = BYTE_COUNT.load(Ordering::SeqCst);
    (count, bytes)
}

pub fn record_metrics(bytes: usize) {
    MESSAGE_COUNT.fetch_add(1, Ordering::SeqCst);
    BYTE_COUNT.fetch_add(bytes, Ordering::SeqCst);
}

// Set up slog for benchmarking
pub fn setup_slog_bench() -> (Logger, Arc<MessageStats>) {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));

    let drain = Async::new(writer.fuse()).build().fuse();

    (Logger::root(drain, o!()), Arc::clone(&stats))
}

// Shutdown helper to close any async resources safely
pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        // First wait to allow slog_async to process its buffer
        std::thread::sleep(std::time::Duration::from_millis(2000));

        // Explicitly flush any remaining logs
        log::logger().flush();

        // Handle slog_async channel shutdown if needed
        if NEEDS_ASYNC_CLOSE.load(Ordering::SeqCst) {
            if let Some(drain) = ASYNC_DRAIN.get() {
                // Give time for any remaining messages to be processed
                std::thread::sleep(std::time::Duration::from_millis(500));
                // The Arc<Async> will handle its own cleanup when dropped
                drop(Arc::clone(drain));
            }
            NEEDS_ASYNC_CLOSE.store(false, Ordering::SeqCst);
        }

        // Handle ftlog shutdown if needed
        if NEEDS_FTLOG_CLOSE.load(Ordering::SeqCst) {
            // Give time for any remaining messages to be processed
            std::thread::sleep(std::time::Duration::from_millis(500));
            // ftlog will handle its own cleanup when the program exits
            NEEDS_FTLOG_CLOSE.store(false, Ordering::SeqCst);
        }

        // Final wait for any remaining loggers
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

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

    // Set the flag to indicate that ftlog needs to be closed
    NEEDS_FTLOG_CLOSE.store(true, Ordering::SeqCst);
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

// ===== Benchmark Logger Setup Functions (for fair performance measurements) =====

// Common benchmark log format: "{timestamp} [{level}] - {target}: {message}"
// Using consistent configuration for all loggers:
// - UTC timestamps with millisecond precision
// - No ANSI colors
// - Same level of context (module/target name)
// - Same log level (Info)
// - All use blocking behavior on buffer overflow

// A writer that counts bytes written
#[derive(Debug)]
pub struct CountingWriter {
    stats: Arc<MessageStats>,
}

impl CountingWriter {
    pub fn new(stats: Arc<MessageStats>) -> Self {
        Self { stats }
    }
}

impl Write for CountingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        self.stats.record_message(len);
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Log for CountingWriter {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let msg = format!("{}", record.args());
        self.stats.record_message(msg.len());
    }

    fn flush(&self) {}
}

impl slog::Drain for CountingWriter {
    type Ok = ();
    type Err = std::io::Error;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> Result<Self::Ok, Self::Err> {
        let mut size = 0;
        record
            .kv()
            .serialize(record, &mut SimpleSerializer(&mut size))?;
        values.serialize(record, &mut SimpleSerializer(&mut size))?;
        let msg = format!("{}", record.msg());
        self.stats.record_message(msg.len() + size);
        Ok(())
    }
}

struct SimpleSerializer<'a>(&'a mut usize);

impl<'a> slog::Serializer for SimpleSerializer<'a> {
    fn emit_arguments(&mut self, key: slog::Key, val: &std::fmt::Arguments) -> slog::Result {
        *self.0 += key.len() + format!("{}", val).len();
        Ok(())
    }
}

// Set up fern for benchmarking
pub fn setup_fern_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));

    let logger = Dispatch::new()
        .format(move |out, message, record| {
            let formatted = format!(
                "{} [{}] - {}: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            );
            out.finish(format_args!("{}", formatted))
        })
        .level(log::LevelFilter::Info)
        .chain(Box::new(writer) as Box<dyn Write + Send>);

    let _ = logger.apply();

    Arc::clone(&stats)
}

// Set up log4rs for benchmarking
pub fn setup_log4rs_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));

    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .target(Target::Stdout)
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("counter", Box::new(appender)))
        .build(
            Root::builder()
                .appender("counter")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    let _ = init_config(config);

    Arc::clone(&stats)
}

// Set up ftlog for benchmarking
pub fn setup_ftlog_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));

    let _ = ftlog::builder()
        .max_log_level(ftlog::LevelFilter::Info)
        .root(ftlog::appender::ChainAppenders::new(vec![
            Box::new(writer) as Box<dyn Write + Send>
        ]))
        .try_init();

    Arc::clone(&stats)
}

// Set up tracing for benchmarking
pub fn setup_tracing_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    // Clone stats for the closure to capture.
    let stats_for_closure = Arc::clone(&stats);

    let subscriber = fmt()
        .with_max_level(TracingLevel::INFO)
        // Use `move` closure. It captures `stats_for_closure`.
        .with_writer(move || {
            // Clone the *captured* Arc inside the closure body.
            Box::new(CountingWriter::new(Arc::clone(&stats_for_closure))) as Box<dyn Write + Send>
        })
        .without_time()
        .with_ansi(false)
        .with_target(true)
        .finish();

    let _ = set_global_default(subscriber);
    // Return the original `stats` Arc.
    stats
}

// Set up env_logger for benchmarking
pub fn setup_env_logger_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));

    env_logger::Builder::new()
        .format(move |_, record| {
            let msg = format!("{}", record.args());
            // Explicitly call the Log trait's log method
            log::Log::log(&writer, record);
            Ok(())
        })
        .filter(None, log::LevelFilter::Info)
        // Use try_init() to avoid panic if logger is already set
        .try_init()
        .ok(); // Ignore the result, proceed even if logger init fails

    // Return the original stats
    Arc::clone(&stats)
}
