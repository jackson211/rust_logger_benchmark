// src/logger_setup/common.rs
use crate::metrics::MessageStats;
use slog_async;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Statics needed by LoggerGuard
static NEEDS_ASYNC_CLOSE: AtomicBool = AtomicBool::new(false);
static ASYNC_DRAIN: std::sync::OnceLock<Arc<slog_async::Async>> = std::sync::OnceLock::new();
static NEEDS_FTLOG_CLOSE: AtomicBool = AtomicBool::new(false);

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

// Shutdown helper
pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        log::logger().flush();

        if NEEDS_ASYNC_CLOSE.load(Ordering::SeqCst) {
            if let Some(drain) = ASYNC_DRAIN.get() {
                std::thread::sleep(std::time::Duration::from_millis(500));
                drop(Arc::clone(drain));
            }
            NEEDS_ASYNC_CLOSE.store(false, Ordering::SeqCst);
        }

        if NEEDS_FTLOG_CLOSE.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(500));
            NEEDS_FTLOG_CLOSE.store(false, Ordering::SeqCst);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
