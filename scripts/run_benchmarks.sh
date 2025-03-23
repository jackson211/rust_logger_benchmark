#!/bin/bash

set -e

function print_header() {
    echo "====================================================="
    echo "  Rust Logging Benchmarks"
    echo "====================================================="
    echo ""
}

function check_prerequisites() {
    # Check if uv is installed
    if ! command -v uv &> /dev/null; then
        echo "Error: 'uv' is not installed. Please install it first."
        echo "Visit: https://github.com/astral-sh/uv"
        exit 1
    fi
}

function run_benchmark() {
    echo "Running benchmarks sequentially (this may take several minutes)..."
    
    # List of benchmarks to run
    benchmarks=(
        "env_logger_bench"
        "fern_bench"
        "ftlog_bench"
        "log4rs_bench"
        "slog_bench"
        "tracing_bench"
    )
    
    # Run each benchmark individually
    for bench in "${benchmarks[@]}"; do
        echo "Running benchmark: $bench"
        cargo bench --bench "$bench"
        echo "Completed: $bench"
        echo "-------------------------------------------"
    done
    
    echo "All benchmarks completed successfully."
}

function generate_results() {
    echo "Generating benchmark results with Python..."
    
    # Create virtual environment and install dependencies
    uv venv .venv
    source .venv/bin/activate
    uv pip install -r scripts/requirements.txt
    
    # Run the Python script
    python scripts/generate_results.py
    
    # Deactivate virtual environment
    deactivate
    
    echo "Results generated successfully in the 'results' directory!"
    echo "- View HTML reports: target/criterion/report/index.html"
    echo "- View markdown summary: results/benchmark_results.md"
}

function main() {
    print_header
    check_prerequisites
    
    # Parse command line arguments
    if [[ "$1" == "--report-only" ]]; then
        echo "Generating reports from existing benchmark data..."
        generate_results
    else
        echo "Running full benchmark suite and generating reports..."
        run_benchmark
        generate_results
    fi
}

# Execute main function
main "$@" 