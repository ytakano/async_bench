use async_bench::{async_std_bench, std_thread, thread_crossbeam, thread_flume, tokio_bench};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::{sync::Arc, time::Duration};
use tokio::runtime::Runtime as TokioRuntime;

fn bench_one_to_one(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("one_to_one"));
    g.measurement_time(Duration::from_secs(200));
    let runtime = Arc::new(TokioRuntime::new().unwrap());

    for i in [1, 2, 4, 6, 8].iter() {
        g.bench_with_input(
            BenchmarkId::new("asyn_std::channel::unbounded", i),
            i,
            |b, i| {
                b.iter(move || {
                    async_std::task::block_on(async {
                        let mut hdl = async_std_bench::new_one_to_one_unbounded(*i as usize).await;
                        hdl.start().await;
                    });
                })
            },
        );

        g.bench_with_input(
            BenchmarkId::new("asyn_std::channel::bounded", i),
            i,
            |b, i| {
                b.iter(move || {
                    async_std::task::block_on(async {
                        let mut hdl = async_std_bench::new_one_to_one_bounded(*i as usize).await;
                        hdl.start().await;
                    });
                })
            },
        );

        let rt = runtime.clone();
        g.bench_with_input(
            BenchmarkId::new("tokio::sync::mpsc::unbounded_channel", i),
            i,
            move |b, i| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut hdl = tokio_bench::new_one_to_one_unbounded(*i as usize).await;
                        hdl.start().await;
                    });
                })
            },
        );

        let rt = runtime.clone();
        g.bench_with_input(
            BenchmarkId::new("tokio::sync::mpsc::channel", i),
            i,
            move |b, i| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut hdl = tokio_bench::new_one_to_one_bounded(*i as usize).await;
                        hdl.start().await;
                    });
                })
            },
        );

        g.bench_with_input(BenchmarkId::new("std::mpsc::channel", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = std_thread::new_channel(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(
            BenchmarkId::new("std::mpsc::new_sync_channel", i),
            i,
            |b, i| {
                b.iter(move || {
                    let mut hdl = std_thread::new_sync_channel(*i as usize);
                    hdl.start();
                })
            },
        );

        g.bench_with_input(BenchmarkId::new("crossbeam::unbounded", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_crossbeam::new_unbounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("crossbeam::bounded", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_crossbeam::new_bounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("flume::unbounded", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_flume::new_unbounded(*i as usize);
                hdl.start();
            })
        });

        g.bench_with_input(BenchmarkId::new("flume::bounded", i), i, |b, i| {
            b.iter(move || {
                let mut hdl = thread_flume::new_bounded(*i as usize);
                hdl.start();
            })
        });
    }
    g.finish();
}

criterion_group!(benches, bench_one_to_one);
criterion_main!(benches);
