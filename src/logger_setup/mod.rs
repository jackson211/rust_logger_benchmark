// src/logger_setup/mod.rs

// Declare modules
pub mod benchmark_setups;
pub mod common;
pub mod standard_setups;

// Re-export public items for easier use
// Common utilities
pub use common::{CountingWriter, LoggerGuard};

// Standard setup functions
pub use standard_setups::{
    setup_env_logger, setup_fern, setup_ftlog, setup_log4rs, setup_slog, setup_tracing,
};

// Benchmark setup functions
pub use benchmark_setups::{
    setup_env_logger_bench, setup_fern_bench, setup_log4rs_bench, setup_slog_bench,
    setup_tracing_bench,
};
