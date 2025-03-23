#!/usr/bin/env python3

import json
import os
import statistics
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any


def main():
    print("Generating benchmark results...")

    # Create results directory if it doesn't exist
    results_dir = Path("results")
    results_dir.mkdir(exist_ok=True)

    # Path to Criterion results
    criterion_dir = Path("target/criterion")

    # Ensure the criterion directory exists
    if not criterion_dir.exists():
        print(
            "Error: Criterion results not found. Please run benchmarks with 'cargo bench' first."
        )
        return 1

    # Collect all benchmark results
    results = {}

    # Process each logger's benchmarks
    loggers = ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]
    for logger in loggers:
        # Process message size benchmarks
        size_dir = criterion_dir / logger
        if size_dir.exists():
            process_benchmark_group(size_dir, logger, results)
        else:
            print(f"Skipping missing benchmark: {logger}")

        # Process log level benchmarks
        levels_dir = criterion_dir / f"{logger}_levels"
        if levels_dir.exists():
            process_benchmark_group(levels_dir, f"{logger}_levels", results)
        else:
            print(f"Skipping missing benchmark: {logger}_levels")

    # Generate markdown report
    generate_markdown_report(results_dir, results)

    # Find fastest, most consistent, and best throughput loggers
    fastest = find_fastest_logger(results)
    most_consistent = find_most_consistent_logger(results)
    best_throughput = find_best_throughput_logger(results)

    print(f"Results successfully generated in {results_dir}/benchmark_results.md")
    print(f"\nKey findings:")
    print(f"- Fastest logger: {fastest}")
    print(f"- Most consistent logger: {most_consistent}")
    print(f"- Best for high throughput: {best_throughput}")

    return 0


def process_benchmark_group(dir_path: Path, group_name: str, results: Dict):
    print(f"Processing benchmark group: {group_name}")

    # Check each entry in the directory
    for entry in dir_path.iterdir():
        if entry.is_dir():
            benchmark_name = entry.name

            # Skip the report directory
            if benchmark_name == "report":
                continue

            # Read the estimates.json file - look in the new subdirectory
            estimates_path = entry / "new" / "estimates.json"
            if estimates_path.exists():
                try:
                    ns_per_iter, throughput = process_benchmark_file(estimates_path)

                    # Store results
                    if group_name not in results:
                        results[group_name] = {}
                    results[group_name][benchmark_name] = (ns_per_iter, throughput)

                except Exception as e:
                    print(f"Error processing benchmark file {estimates_path}: {e}")
            else:
                print(f"No estimates file found at: {estimates_path}")


def process_benchmark_file(file_path: Path) -> Tuple[float, Optional[float]]:
    # Read the estimates.json file
    with open(file_path, "r") as f:
        estimates = json.load(f)

    # Extract measurements - use the slope's point_estimate as it's more stable
    ns_per_iter = estimates["slope"]["point_estimate"]

    # Calculate throughput (logs per second)
    # 1 second = 1,000,000,000 nanoseconds
    throughput = 1_000_000_000.0 / ns_per_iter

    return ns_per_iter, throughput


def generate_markdown_report(results_dir: Path, results: Dict):
    output_file = results_dir / "benchmark_results.md"
    print(f"Generating markdown report: {output_file}")

    with open(output_file, "w") as md:
        md.write("# Logging Library Benchmark Results\n\n")
        md.write(
            "This document contains benchmark results for various Rust logging libraries.\n\n"
        )

        # Create performance tables by benchmark type
        generate_message_size_tables(md, results)
        generate_log_level_tables(md, results)

        # Generate charts section (these will be links to the Criterion HTML reports)
        md.write("## Interactive Charts\n\n")
        md.write(
            "For interactive charts, please view the Criterion HTML reports generated in `target/criterion/report/index.html`.\n\n"
        )


def generate_message_size_tables(md, results: Dict):
    md.write("## Message Size Benchmarks\n\n")
    md.write(
        "These benchmarks measure the performance of logging messages of different sizes.\n\n"
    )

    # Table for nanoseconds per iteration
    md.write("### Time per Log (ns)\n\n")
    md.write("| Logger | 10 byte message | 100 byte message | 1000 byte message |\n")
    md.write("|--------|----------------|------------------|-------------------|\n")

    for logger in ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]:
        if logger in results:
            benchmarks = results[logger]
            md.write(f"| {logger} ")

            for size in ["10", "100", "1000"]:
                if size in benchmarks:
                    ns, _ = benchmarks[size]
                    md.write(f"| {ns:.2f} ")
                else:
                    md.write("| N/A ")

            md.write("|\n")

    md.write("\n")

    # Table for throughput
    md.write("### Throughput (logs/second)\n\n")
    md.write("| Logger | 10 byte message | 100 byte message | 1000 byte message |\n")
    md.write("|--------|----------------|------------------|-------------------|\n")

    for logger in ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]:
        if logger in results:
            benchmarks = results[logger]
            md.write(f"| {logger} ")

            for size in ["10", "100", "1000"]:
                if size in benchmarks:
                    _, throughput = benchmarks[size]
                    if throughput is not None:
                        md.write(f"| {throughput:.2f} ")
                    else:
                        md.write("| N/A ")
                else:
                    md.write("| N/A ")

            md.write("|\n")

    md.write("\n")


def generate_log_level_tables(md, results: Dict):
    md.write("## Log Level Benchmarks\n\n")
    md.write(
        "These benchmarks measure the performance of logging at different levels.\n\n"
    )

    # Table for nanoseconds per iteration
    md.write("### Time per Log (ns)\n\n")
    md.write("| Logger | trace | debug | info | warn | error |\n")
    md.write("|--------|-------|-------|------|------|-------|\n")

    logger_levels = [
        "env_logger_levels",
        "fern_levels",
        "ftlog_levels",
        "log4rs_levels",
        "slog_levels",
        "tracing_levels",
    ]

    for logger in logger_levels:
        if logger in results:
            benchmarks = results[logger]
            md.write(f"| {logger.replace('_levels', '')} ")

            for level in ["trace", "debug", "info", "warn", "error"]:
                if level in benchmarks:
                    ns, _ = benchmarks[level]
                    md.write(f"| {ns:.2f} ")
                else:
                    md.write("| N/A ")

            md.write("|\n")

    md.write("\n")

    # Table for throughput
    md.write("### Throughput (logs/second)\n\n")
    md.write("| Logger | trace | debug | info | warn | error |\n")
    md.write("|--------|-------|-------|------|------|-------|\n")

    for logger in logger_levels:
        if logger in results:
            benchmarks = results[logger]
            md.write(f"| {logger.replace('_levels', '')} ")

            for level in ["trace", "debug", "info", "warn", "error"]:
                if level in benchmarks:
                    _, throughput = benchmarks[level]
                    if throughput is not None:
                        md.write(f"| {throughput:.2f} ")
                    else:
                        md.write("| N/A ")
                else:
                    md.write("| N/A ")

            md.write("|\n")

    md.write("\n")


def find_fastest_logger(results: Dict) -> str:
    fastest = None
    fastest_time = float("inf")

    # Check message size benchmarks (100 byte messages)
    for logger in ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]:
        if logger in results and "100" in results[logger]:
            time, _ = results[logger]["100"]
            if time < fastest_time:
                fastest_time = time
                fastest = logger

    return fastest or "unknown"


def find_most_consistent_logger(results: Dict) -> str:
    most_consistent = None
    min_variance = float("inf")

    # Check performance variance across message sizes
    for logger in ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]:
        if logger in results:
            times = []
            for size in ["10", "100", "1000"]:
                if size in results[logger]:
                    time, _ = results[logger][size]
                    times.append(time)

            if len(times) == 3:
                # Calculate normalized standard deviation as a measure of consistency
                mean = sum(times) / len(times)
                variance = sum((x - mean) ** 2 for x in times) / len(times)
                normalized_variance = (variance**0.5 / mean) * 100.0  # percentage

                if normalized_variance < min_variance:
                    min_variance = normalized_variance
                    most_consistent = logger

    return most_consistent or "unknown"


def find_best_throughput_logger(results: Dict) -> str:
    best = None
    max_throughput = 0.0

    # Check throughput for 100-byte messages
    for logger in ["env_logger", "fern", "ftlog", "log4rs", "slog", "tracing"]:
        if logger in results and "100" in results[logger]:
            _, throughput = results[logger]["100"]
            if throughput is not None and throughput > max_throughput:
                max_throughput = throughput
                best = logger

    return best or "unknown"


if __name__ == "__main__":
    exit(main())
