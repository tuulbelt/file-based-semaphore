//! File-Based Semaphore Benchmarks
//!
//! Measures performance of core operations using Criterion for statistical rigor.
//!
//! Run: cargo bench
//!
//! See: /docs/BENCHMARKING_STANDARDS.md

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;
use tempfile::TempDir;

fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

fn semaphore_creation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Semaphore Creation");

    group.bench_function("create: basic", |b| {
        let temp_dir = create_temp_dir();
        let config = SemaphoreConfig::default();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let path = temp_dir.path().join(format!("bench-{}.lock", counter));
            black_box(Semaphore::new(&path, config.clone()))
        })
    });

    group.bench_function("create: with custom timeout", |b| {
        let temp_dir = create_temp_dir();
        let config = SemaphoreConfig {
            acquire_timeout: Some(Duration::from_secs(5)),
            ..Default::default()
        };
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let path = temp_dir.path().join(format!("bench-{}.lock", counter));
            black_box(Semaphore::new(&path, config.clone()))
        })
    });

    group.finish();
}

fn lock_operations_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lock Operations");

    group.bench_function("acquire + release: no contention", |b| {
        let temp_dir = create_temp_dir();
        let config = SemaphoreConfig::default();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let path = temp_dir.path().join(format!("lock-{}.lock", counter));
            let sem = Semaphore::new(&path, config.clone()).unwrap();
            let _guard = sem.acquire().unwrap();
            black_box(())
        })
    });

    group.finish();
}

fn lock_file_io_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lock File I/O");

    group.bench_function("check: lock exists", |b| {
        let temp_dir = create_temp_dir();
        let path = temp_dir.path().join("io-bench.lock");
        let config = SemaphoreConfig::default();
        let sem = Semaphore::new(&path, config).unwrap();
        b.iter(|| black_box(sem.is_locked()))
    });

    group.finish();
}

criterion_group!(
    benches,
    semaphore_creation_benchmarks,
    lock_operations_benchmarks,
    lock_file_io_benchmarks
);
criterion_main!(benches);
