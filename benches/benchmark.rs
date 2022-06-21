use async_bench::{std_thread, thread_crossbeam, thread_flume};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn std_thread_mpsc(c: &mut Criterion) {
    let mut g = c.benchmark_group("std::thread (mpsc): one_to_one");

    g.measurement_time(Duration::from_secs(300));

    for i in [8, 6, 4, 2, 1] {
        g.bench_function(format!("n = {i}"), |b| {
            // let mut hdl = ;
            // b.iter(move || hdl.start());
            b.iter(move || {
                let mut hdl = std_thread::OneToOne::new(i);
                hdl.start();
            });
        });
    }
}

fn thread_flume(c: &mut Criterion) {
    let mut g = c.benchmark_group("std::thread (mpsc): one_to_one");

    g.measurement_time(Duration::from_secs(300));

    for i in [8, 6, 4, 2, 1] {
        g.bench_function(format!("n = {i}"), |b| {
            // let mut hdl = ;
            // b.iter(move || hdl.start());
            b.iter(move || {
                let mut hdl = thread_flume::OneToOne::new(i);
                hdl.start();
            });
        });
    }
}

fn thread_crossbeam(c: &mut Criterion) {
    let mut g = c.benchmark_group("std::thread (mpsc): one_to_one");

    g.measurement_time(Duration::from_secs(300));

    for i in [8, 6, 4, 2, 1] {
        g.bench_function(format!("n = {i}"), |b| {
            // let mut hdl = ;
            // b.iter(move || hdl.start());
            b.iter(move || {
                let mut hdl = thread_crossbeam::OneToOne::new(i);
                hdl.start();
            });
        });
    }
}

criterion_group!(benches, std_thread_mpsc, thread_flume, thread_crossbeam);
criterion_main!(benches);
