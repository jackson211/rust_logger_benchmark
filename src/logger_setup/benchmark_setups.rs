// src/logger_setup/benchmark_setups.rs
use crate::logger_setup::common::CountingWriter; // Import from local common module
use crate::metrics::MessageStats;
use chrono::Utc;
use fern::Dispatch;
use log4rs::{
    append::Append,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    encode::writer::simple::SimpleWriter,
    encode::Encode,
    init_config,
};
use slog::{o, Drain, Logger};
use slog_term::{FullFormat, PlainSyncDecorator};
use std::io::Write;
use std::sync::{Arc, Mutex};
use time::format_description::well_known::Rfc3339;
use tracing::Level as TracingLevel;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::{FmtSpan, Format};
use tracing_subscriber::fmt::time::UtcTime;

// ===== Benchmark Logger Setup Functions =====

// --- slog ---
pub fn setup_slog_bench() -> (Logger, Arc<MessageStats>) {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));
    let decorator = PlainSyncDecorator::new(writer);
    let drain = FullFormat::new(decorator)
        .use_utc_timestamp()
        .build()
        .fuse();
    (Logger::root(drain, o!()), Arc::clone(&stats))
}

// --- log4rs ---
#[derive(Debug)]
struct CountingAppender {
    writer: Arc<Mutex<CountingWriter>>,
    encoder: PatternEncoder,
}

impl CountingAppender {
    fn new(stats: Arc<MessageStats>) -> Self {
        let writer = Arc::new(Mutex::new(CountingWriter::new(stats)));
        let encoder = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)(utc)} [{l}] - {t}: {m}{n}");
        CountingAppender { writer, encoder }
    }
}

impl Append for CountingAppender {
    fn append(&self, record: &log::Record) -> anyhow::Result<()> {
        let buf = Vec::new();
        let mut simple_writer = SimpleWriter(buf);
        self.encoder.encode(&mut simple_writer, record)?;
        let mut writer_guard = self.writer.lock().unwrap();
        writer_guard.write_all(simple_writer.0.as_slice())?;
        Ok(())
    }
    fn flush(&self) {}
}

pub fn setup_log4rs_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let appender = CountingAppender::new(Arc::clone(&stats));
    let config = Config::builder()
        .appender(Appender::builder().build("counter", Box::new(appender)))
        .build(
            Root::builder()
                .appender("counter")
                .build(log::LevelFilter::Info),
        )
        .unwrap();
    let _ = init_config(config); // Potential panic
    Arc::clone(&stats)
}

// --- fern ---
pub fn setup_fern_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = CountingWriter::new(Arc::clone(&stats));
    let _logger = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {}: {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(Box::new(writer) as Box<dyn Write + Send>)
        .apply(); // Potential panic
    Arc::clone(&stats)
}

// --- tracing ---
pub fn setup_tracing_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let stats_for_closure = Arc::clone(&stats);

    let format = Format::default()
        .with_timer(UtcTime::new(Rfc3339))
        .with_level(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .compact();

    let subscriber = fmt()
        .event_format(format)
        .with_max_level(TracingLevel::INFO)
        .with_writer(move || {
            Box::new(CountingWriter::new(Arc::clone(&stats_for_closure))) as Box<dyn Write + Send>
        })
        .with_ansi(false)
        .with_span_events(FmtSpan::NONE)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);

    stats
}

// --- env_logger ---
pub fn setup_env_logger_bench() -> Arc<MessageStats> {
    let stats = Arc::new(MessageStats::new());
    let writer = Arc::new(Mutex::new(CountingWriter::new(Arc::clone(&stats))));
    env_logger::Builder::new()
        .format(move |_buf, record| {
            let formatted_msg = format!(
                "{} [{}] - {}: {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.args()
            );
            let mut writer_guard = writer.lock().unwrap();
            writeln!(writer_guard, "{}", formatted_msg)
        })
        .filter(None, log::LevelFilter::Info)
        .try_init()
        .ok();
    Arc::clone(&stats)
}
