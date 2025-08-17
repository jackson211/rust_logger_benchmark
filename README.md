# Rust Logging Crates Benchmark

This project compares the performance of various logging crates in the Rust ecosystem. The benchmarks measure different aspects of logging performance including message size, log levels, and throughput.

## Test Environment

All benchmarks were run on:

## Results

After running the benchmarks using `cargo bench`, the results are stored in the `target/criterion` directory.

- **Hardware**: Mac Mini M4 (Apple Silicon)
- **Memory**: 24GB RAM
- **OS**: macOS Sequoia
- **Rust**: 1.85.0

## Included Logging Crates

- **log** with **env_logger**: The standard logging facade for Rust
- **tracing**: A framework for instrumenting Rust programs to collect structured, event-based diagnostic information
- **slog**: Structured, contextual, extensible, composable logging for Rust
- **log4rs**: A highly configurable logging framework modeled after Java's log4j and logback
- **fern**: A simple, efficient logging implementation
- **ftlog**: Fast, zero-allocation logging library optimized for high performance

## Running the Demo

To run a simple demonstration of all logging crates:

```bash
cargo run
```

This will show examples of using each logging crate with various log messages.

## Running the Benchmarks

To run the benchmarks and compare performance:

```bash
cargo bench
```

The benchmarks measure:

1. **Message Size**: Performance with different log message sizes (10, 100, 1000 characters)
2. **Log Levels**: Performance differences between trace, debug, info, warn, and error levels

## Benchmark Methodology

The benchmarks ensure fair comparison across all logging frameworks by:

- Using consistent log format across all loggers
- Standardizing on UTC timestamps with millisecond precision
- Disabling ANSI colors for all loggers
- Setting equivalent log level (Info) and message handling behavior
- Measuring formatted message sizes consistently
- Using blocking behavior for all loggers

This methodology allows for direct comparison of the core performance characteristics of each logging framework without being affected by differences in formatting or configuration.

## Benchmark Aspects

The benchmarks are designed to measure:

- **Throughput**: How many log messages can be processed per second
- **Latency**: How long it takes to process a single log message
- **Effect of Message Size**: How performance scales with increased message size
- **Log Level Performance**: Performance differences between different log levels

## Customizing Benchmarks

You can modify the benchmark parameters in the `benches/common.rs` file and individual benchmark files in the `benches/` directory.

## Notes

- The benchmarks are run with output loggers enabled, which affects performance
- Some logging crates (particularly log implementations) may show similar performance because they use the same underlying facade

## Performance Comparison

Benchmark results comparing various Rust logging libraries are available after running the benchmarks. The consolidated report makes it easy to compare the performance characteristics of each logger.

### Key Metrics

For each logger, the benchmarks report:

- Message count
- Total bytes processed
- Average bytes per message
- Performance in terms of throughput and latency
