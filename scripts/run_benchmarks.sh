#!/bin/bash

# Run all benchmarks script
# This script runs all the logging benchmarks and saves the results

# Set up colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory if it doesn't exist
mkdir -p results

# Get current date and time for the results file
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULTS_FILE="results/benchmark_results_${TIMESTAMP}.txt"

echo -e "${BLUE}Running all logging benchmarks...${NC}"
echo "Logging benchmark results - $(date)" > "$RESULTS_FILE"
echo "=====================================" >> "$RESULTS_FILE"

# Function to run a benchmark and append results
run_benchmark() {
    local name=$1
    echo -e "${GREEN}Running $name benchmark...${NC}"
    echo -e "\n=== $name Benchmark ===" >> "$RESULTS_FILE"
    cargo bench --bench "$name" >> "$RESULTS_FILE" 2>&1
    echo "=====================================" >> "$RESULTS_FILE"
}

# Run each benchmark
run_benchmark "slog_bench"
run_benchmark "log4rs_bench"
run_benchmark "fern_bench"
run_benchmark "ftlog_bench"
run_benchmark "tracing_bench"

echo -e "${BLUE}All benchmarks completed. Results saved to: ${RESULTS_FILE}${NC}"
echo -e "${GREEN}Done!${NC}" 