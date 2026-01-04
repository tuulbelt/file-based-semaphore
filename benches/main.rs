//! File-Based Semaphore Benchmarks
//!
//! Measures performance of core operations using Criterion for statistical rigor.
//!
//! Run: cargo bench
//!
//! See: /docs/BENCHMARKING_STANDARDS.md

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

fn semaphore_creation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Semaphore Creation");

    group.bench_function("create: basic", |b| {
        let temp_dir = create_temp_dir();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let config = SemaphoreConfig {
                name: format!("bench-{}", counter),
                lock_dir: temp_dir.path().to_path_buf(),
                ..Default::default()
            };
            black_box(Semaphore::new(config))
        })
    });

    group.bench_function("create: with timeout", |b| {
        let temp_dir = create_temp_dir();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let config = SemaphoreConfig {
                name: format!("bench-{}", counter),
                lock_dir: temp_dir.path().to_path_buf(),
                timeout_ms: Some(5000),
                ..Default::default()
            };
            black_box(Semaphore::new(config))
        })
    });

    group.finish();
}

fn lock_operations_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lock Operations");

    group.bench_function("acquire + release: no contention", |b| {
        let temp_dir = create_temp_dir();
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let config = SemaphoreConfig {
                name: format!("lock-{}", counter),
                lock_dir: temp_dir.path().to_path_buf(),
                ..Default::default()
            };
            let sem = Semaphore::new(config).unwrap();
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
        let config = SemaphoreConfig {
            name: "io-bench".to_string(),
            lock_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let sem = Semaphore::new(config).unwrap();
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
