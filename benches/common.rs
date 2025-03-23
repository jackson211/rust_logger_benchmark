use criterion::{BenchmarkId, Criterion, Throughput, black_box};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Generate random log message of specified length
/// Uses a fixed seed to generate the same message every time
pub fn generate_random_message(length: usize) -> String {
    // Use a fixed seed (42) for consistent output across runs
    let mut rng = StdRng::seed_from_u64(42);
    (0..length)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)
        .collect()
}

/// Run benchmark with different message sizes
pub fn bench_message_sizes<F>(c: &mut Criterion, group_name: &str, log_fn: F)
where
    F: Fn(&str) + Copy,
{
    let mut group = c.benchmark_group(group_name);
    for size in [10, 100, 1000].iter() {
        // Set the throughput to 1 operation per iteration
        // This will measure logs per second in the output
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let message = generate_random_message(size);
            b.iter(|| {
                black_box(log_fn(black_box(&message)));
            });
        });
    }
    group.finish();
}

/// Run benchmark for different log levels
pub fn bench_log_levels<F1, F2, F3, F4, F5>(
    c: &mut Criterion,
    group_name: &str,
    trace_fn: F1,
    debug_fn: F2,
    info_fn: F3,
    warn_fn: F4,
    error_fn: F5,
) where
    F1: Fn(&str),
    F2: Fn(&str),
    F3: Fn(&str),
    F4: Fn(&str),
    F5: Fn(&str),
{
    let mut group = c.benchmark_group(group_name);
    let message = generate_random_message(100);

    // Set the throughput to 1 operation per iteration
    // This will measure logs per second in the output
    group.throughput(Throughput::Elements(1));

    group.bench_function("trace", |b| {
        b.iter(|| {
            black_box(trace_fn(black_box(&message)));
        });
    });

    group.bench_function("debug", |b| {
        b.iter(|| {
            black_box(debug_fn(black_box(&message)));
        });
    });

    group.bench_function("info", |b| {
        b.iter(|| {
            black_box(info_fn(black_box(&message)));
        });
    });

    group.bench_function("warn", |b| {
        b.iter(|| {
            black_box(warn_fn(black_box(&message)));
        });
    });

    group.bench_function("error", |b| {
        b.iter(|| {
            black_box(error_fn(black_box(&message)));
        });
    });

    group.finish();
}
