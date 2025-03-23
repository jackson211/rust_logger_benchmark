# Rust Logging Crates Benchmark

This project compares the performance of various logging crates in the Rust ecosystem. The benchmarks measure different aspects of logging performance including message size, log levels, and throughput.

## Test Environment

All benchmarks were run on:

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
./benchmark.sh
```

The benchmarks measure:

1. **Message Size**: Performance with different log message sizes (10, 100, 1000 characters)
2. **Log Levels**: Performance differences between trace, debug, info, warn, and error levels

### Generating Results

You can generate benchmark results in readable format:

```bash
# Run benchmarks and generate results
./benchmark.sh

# OR if you've already run benchmarks, just generate the report
./benchmark.sh --report-only
```

This will:

1. Create a `results` directory with markdown reports
2. Generate formatted benchmark data for easy comparison

The results include both timing measurements (nanoseconds per log) and throughput (logs per second) for each logger.

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

Benchmark results comparing various Rust logging libraries are available in the [results/benchmark_results.md](results/benchmark_results.md) file.

### Key Findings

- **Fastest Logger**: Based on the benchmarks, the fastest logger for most common use cases appears to be **slog**.

- **Most Consistent**: **ftlog** shows the most consistent performance across different message sizes and log levels.

- **Best for High Throughput**: **slog** demonstrates the best performance for high throughput logging scenarios.

- **Memory Usage**: Memory usage patterns vary significantly between logging libraries, with some async implementations using more memory for better throughput.

Please see the full benchmark results for detailed comparisons and to determine which logger best suits your specific needs.
