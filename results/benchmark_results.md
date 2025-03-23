# Logging Library Benchmark Results

This document contains benchmark results for various Rust logging libraries.

## Message Size Benchmarks

These benchmarks measure the performance of logging messages of different sizes.

### Time per Log (ns)

| Logger | 10 byte message | 100 byte message | 1000 byte message |
|--------|----------------|------------------|-------------------|
| env_logger | 3640.22 | 13519.32 | 92334.04 |
| fern | 3888.57 | 11777.73 | 90513.74 |
| ftlog | 178.31 | 175.93 | 196.52 |
| log4rs | 3614.77 | 11868.81 | 91443.89 |
| slog | 73.04 | 73.53 | 91.77 |
| tracing | 8797.12 | 18502.08 | 98468.43 |

### Throughput (logs/second)

| Logger | 10 byte message | 100 byte message | 1000 byte message |
|--------|----------------|------------------|-------------------|
| env_logger | 274708.51 | 73968.21 | 10830.24 |
| fern | 257163.79 | 84906.04 | 11048.05 |
| ftlog | 5608233.36 | 5684080.37 | 5088411.98 |
| log4rs | 276642.65 | 84254.46 | 10935.67 |
| slog | 13691341.61 | 13599441.75 | 10897043.22 |
| tracing | 113673.55 | 54047.97 | 10155.54 |

## Log Level Benchmarks

These benchmarks measure the performance of logging at different levels.

### Time per Log (ns)

| Logger | trace | debug | info | warn | error |
|--------|-------|-------|------|------|-------|
| env_logger | 0.49 | 0.50 | 12014.80 | 11952.72 | 12164.15 |
| fern | 0.47 | 0.50 | 11860.06 | 11756.88 | 12050.93 |
| ftlog | 0.55 | 0.50 | 179.12 | 179.49 | 180.39 |
| log4rs | 0.50 | 0.50 | 11723.99 | 11612.13 | 11692.95 |
| slog | 0.25 | 0.25 | 73.83 | 73.26 | 74.64 |
| tracing | 0.53 | 0.53 | 16508.33 | 16804.00 | 16760.40 |

### Throughput (logs/second)

| Logger | trace | debug | info | warn | error |
|--------|-------|-------|------|------|-------|
| env_logger | 2037404475.46 | 2014351364.26 | 83230.68 | 83662.99 | 82208.77 |
| fern | 2108566297.22 | 2008554873.13 | 84316.62 | 85056.61 | 82981.12 |
| ftlog | 1829122542.31 | 2004263733.28 | 5582845.42 | 5571327.82 | 5543527.18 |
| log4rs | 2001138702.75 | 2005668181.41 | 85295.17 | 86116.86 | 85521.60 |
| slog | 4035426450.05 | 4016633330.96 | 13543939.75 | 13650792.96 | 13397528.71 |
| tracing | 1891395673.53 | 1892990109.68 | 60575.50 | 59509.63 | 59664.45 |

## Interactive Charts

For interactive charts, please view the Criterion HTML reports generated in `target/criterion/report/index.html`.

