#!/bin/bash

# Simple wrapper script for running benchmarks
# Usage: 
#   ./benchmark.sh        # Run benchmarks and generate results
#   ./benchmark.sh --report-only  # Generate reports from existing benchmark data

set -e

# Forward all arguments to the actual script
scripts/run_benchmarks.sh "$@" 