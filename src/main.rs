use std::time::SystemTime;
use tokio::runtime::Runtime as TokioRuntime;

mod async_std_bench;
mod std_thread;
mod thread_crossbeam;
mod thread_flume;
mod tokio_bench;

const MAX_COUNT: usize = 1000000;
const EVAL_RANGE: &[usize] = &[1, 4, 8, 12, 16, 20, 24];

fn main() {
    println!("one-to-one");
    println!("std::thread (std::sync::mpsc::channel)");
    run_one_to_one(std_thread::new_one_to_one_channel);

    println!("std::thread (std::sync::mpsc::sync_channel)");
    run_one_to_one(std_thread::new_one_to_one_sync_channel);

    println!("std::thread (flume::unbounded)");
    run_one_to_one(thread_flume::new_one_to_one_unbounded);

    println!("std::thread (flume::bounded)");
    run_one_to_one(thread_flume::new_one_to_one_bounded);

    println!("std::thread (crossbeam::channel::unbounded)");
    run_one_to_one(thread_crossbeam::new_one_to_one_unbounded);

    println!("std::thread (crossbeam::channel::bounded)");
    run_one_to_one(thread_crossbeam::new_one_to_one_bounded);

    println!("async_std (async_std::channel::unbounded)");
    for i in EVAL_RANGE {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_unbounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                (MAX_COUNT as f64 / sec) as usize
            );
        });
    }

    println!("async_std (async_std::channel::bounded)");
    for i in EVAL_RANGE {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_bounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                (MAX_COUNT as f64 / sec) as usize
            );
        });
    }

    let runtime = TokioRuntime::new().unwrap();

    println!("tokio (tokio::sync::mpsc::unbounded_channel)");
    for i in EVAL_RANGE {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_unbounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                (MAX_COUNT as f64 / sec) as usize
            );
        });
    }

    println!("tokio (tokio::sync::mpsc::channel)");
    for i in EVAL_RANGE {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_bounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                (MAX_COUNT as f64 / sec) as usize
            );
        });
    }

    println!();
    println!("many-to-one");
    println!("std::thread (std::sync::mpsc::sync_channel)");

    run_many_to_one(std_thread::new_many_to_one_sync_channel);

    println!("std::thread (flume::bounded)");
    run_many_to_one(thread_flume::new_many_to_one_bounded);

    println!("std::thread (crossbeam::channel::bounded)");
    run_many_to_one(thread_crossbeam::new_many_to_one_bounded);

    println!("async_std (async_std::channel::bounded)");
    for i in EVAL_RANGE[1..].iter() {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_many_to_one_bounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                ((MAX_COUNT / 10 * *i) as f64 / sec) as usize
            );
        });
    }

    println!("tokio (tokio::sync::mpsc::channel)");
    for i in EVAL_RANGE[1..].iter() {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_many_to_one_bounded(*i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                ((MAX_COUNT / 10 * *i) as f64 / sec) as usize
            );
        });
    }

    println!();
    println!("mutex");
    println!("std::sync::Mutex");
    for i in [4, 8, 12, 16, 20, 24] {
        let mut hdl = std_thread::MutexBench::new(i);
        let start = SystemTime::now();
        hdl.start();
        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {:>2}: {:>10} [ops/s]",
            i,
            ((MAX_COUNT * i) as f64 / sec) as usize
        );
    }

    println!("parking_lot::Mutex");
    for i in [4, 8, 12, 16, 20, 24] {
        let mut hdl = std_thread::MutexBenchPackingLot::new(i);
        let start = SystemTime::now();
        hdl.start();
        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {:>2}: {:>10} [ops/s]",
            i,
            ((MAX_COUNT * i) as f64 / sec) as usize
        );
    }

    println!("async_std::sync::Mutex");
    for i in [4, 8, 12, 16, 20, 24] {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::MutexBench::new(i);
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                ((MAX_COUNT * i) as f64 / sec) as usize
            );
        });
    }

    println!("tokio::sync::Mutex");
    for i in [4, 8, 12, 16, 20, 24] {
        runtime.block_on(async {
            let mut hdl = tokio_bench::MutexBench::new(i);
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!(
                "n = {:>2}: {:>10} [ops/s]",
                i,
                ((MAX_COUNT * i) as f64 / sec) as usize
            );
        });
    }
}

fn run_one_to_one(f: fn(usize) -> std_thread::OneToOne) {
    for i in EVAL_RANGE {
        let mut h = f(*i);
        let start = SystemTime::now();

        h.start();

        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {:>2}: {:>10} [ops/s]",
            i,
            (MAX_COUNT as f64 / sec) as usize
        );
    }
}

fn run_many_to_one(f: fn(usize) -> std_thread::ManyToOne) {
    for i in EVAL_RANGE[1..].iter() {
        let mut h = f(*i);
        let start = SystemTime::now();

        h.start();

        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {:>2}: {:>10} [ops/s]",
            i,
            ((MAX_COUNT / 10 * *i) as f64 / sec) as usize
        );
    }
}
