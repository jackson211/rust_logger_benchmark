# Benchmark Scripts

This directory contains scripts for running and analyzing the logging benchmarks.

## Available Scripts

### run_benchmarks.sh

A simple shell script that runs all the logging benchmarks and saves the results to a timestamped file in the `results` directory.

#### Usage

```bash
# Run all benchmarks
./scripts/run_benchmarks.sh
```

#### Features

- Runs all benchmarks (slog_bench, log4rs_bench, fern_bench, ftlog_bench, tracing_bench)
- Saves results to a timestamped file in the `results` directory
- Provides colored output for better readability
- Captures both stdout and stderr in the results file

#### Output

The script creates a file in the `results` directory with the format:

```
benchmark_results_YYYYMMDD_HHMMSS.txt
```

Each benchmark's results are clearly separated in the output file.
