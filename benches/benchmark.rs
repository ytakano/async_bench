use async_bench::{async_std_bench, std_thread, thread_crossbeam, thread_flume, tokio_bench};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{sync::Arc, time::Duration};
use tokio::runtime::Runtime as TokioRuntime;

fn bench_one_to_one_unbounded(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("1 to 1 (unbounded)"));
    g.measurement_time(Duration::from_secs(30));
    let runtime = Arc::new(TokioRuntime::new().unwrap());

    for i in [1, 4, 8, 12, 16, 20, 24].iter() {
        g.bench_with_input(BenchmarkId::new("async_std", i), i, |b, i| {
            b.iter(move || {
                async_std::task::block_on(async {
                    let mut hdl = async_std_bench::new_one_to_one_unbounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        let rt = runtime.clone();
        g.bench_with_input(BenchmarkId::new("tokio", i), i, move |b, i| {
            b.iter(|| {
                rt.block_on(async {
                    let mut hdl = tokio_bench::new_one_to_one_unbounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        g.bench_with_input(BenchmarkId::new("std", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::new_one_to_one_channel(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("crossbeam", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_crossbeam::new_one_to_one_unbounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("flume", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_flume::new_one_to_one_unbounded(*i as usize);
                hdl.start();
            })
        });
    }
    g.finish();
}

fn bench_one_to_one_bounded(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("1 to 1 (bounded)"));
    g.measurement_time(Duration::from_secs(30));
    let runtime = Arc::new(TokioRuntime::new().unwrap());

    for i in [1, 4, 8, 12, 16, 20, 24].iter() {
        g.bench_with_input(BenchmarkId::new("async_std", i), i, |b, i| {
            b.iter(move || {
                async_std::task::block_on(async {
                    let mut hdl = async_std_bench::new_one_to_one_bounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        let rt = runtime.clone();
        g.bench_with_input(BenchmarkId::new("tokio", i), i, move |b, i| {
            b.iter(|| {
                rt.block_on(async {
                    let mut hdl = tokio_bench::new_one_to_one_bounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        g.bench_with_input(BenchmarkId::new("std", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::new_one_to_one_sync_channel(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("crossbeam", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_crossbeam::new_one_to_one_bounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("flume", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_flume::new_one_to_one_bounded(*i as usize);
                hdl.start();
            })
        });
    }
    g.finish();
}

fn bench_many_to_one(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("many to 1 (bounded)"));
    g.measurement_time(Duration::from_secs(60));
    let runtime = Arc::new(TokioRuntime::new().unwrap());

    for i in [4, 8, 12, 16, 20, 24].iter() {
        g.bench_with_input(BenchmarkId::new("async_std", i), i, |b, i| {
            b.iter(move || {
                async_std::task::block_on(async {
                    let mut hdl = async_std_bench::new_many_to_one_bounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        let rt = runtime.clone();
        g.bench_with_input(BenchmarkId::new("tokio", i), i, move |b, i| {
            b.iter(|| {
                rt.block_on(async {
                    let mut hdl = tokio_bench::new_many_to_one_bounded(*i as usize).await;
                    hdl.start().await;
                });
            })
        });

        g.bench_with_input(BenchmarkId::new("std", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::new_many_to_one_sync_channel(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("crossbeam", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_crossbeam::new_many_to_one_bounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("flume", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_flume::new_many_to_one_bounded(*i as usize);
                hdl.start();
            })
        });
    }

    g.finish();
}

fn bench_mutex(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("mutex"));
    g.measurement_time(Duration::from_secs(120));
    let runtime = Arc::new(TokioRuntime::new().unwrap());

    for i in [4, 6, 8, 12, 16].iter() {
        g.bench_with_input(BenchmarkId::new("std", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::MutexBench::new(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("parking_lot", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::MutexBenchPackingLot::new(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("async_std", i), i, |b, i| {
            b.iter(move || {
                async_std::task::block_on(async {
                    let mut hdl = async_std_bench::MutexBench::new(*i as usize);
                    hdl.start().await;
                });
            })
        });

        let rt = runtime.clone();
        g.bench_with_input(BenchmarkId::new("tokio", i), i, move |b, i| {
            b.iter(|| {
                rt.block_on(async {
                    let mut hdl = tokio_bench::MutexBench::new(*i as usize);
                    hdl.start().await;
                });
            })
        });
    }
}

criterion_group!(
    benches,
    bench_mutex,
    bench_many_to_one,
    bench_one_to_one_unbounded,
    bench_one_to_one_bounded
);
criterion_main!(benches);
