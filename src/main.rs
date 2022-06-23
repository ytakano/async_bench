use std::time::SystemTime;
use tokio::runtime::Runtime as TokioRuntime;

mod async_std_bench;
mod std_thread;
mod thread_crossbeam;
mod thread_flume;
mod tokio_bench;

const MAX_COUNT: usize = 10000000;
const RECV_NUM: usize = 8;

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
    for i in 1..=8 {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_unbounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!("n = {i}: {:>10} [ops/s]", (MAX_COUNT as f64 / sec) as usize);
        });
    }

    println!("async_std (async_std::channel::bounded)");
    for i in 1..=8 {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_bounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!("n = {i}: {:>10} [ops/s]", (MAX_COUNT as f64 / sec) as usize);
        });
    }

    let runtime = TokioRuntime::new().unwrap();

    println!("tokio (tokio::sync::mpsc::unbounded_channel)");
    for i in 1..=8 {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_unbounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!("n = {i}: {:>10} [ops/s]", (MAX_COUNT as f64 / sec) as usize);
        });
    }

    println!("tokio (tokio::sync::mpsc::channel)");
    for i in 1..=8 {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_bounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            let dur = start.elapsed().unwrap();
            let sec = dur.as_secs_f64();
            println!("n = {i}: {:>10} [ops/s]", (MAX_COUNT as f64 / sec) as usize);
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
    async_std::task::block_on(async {
        let mut hdl = async_std_bench::new_many_to_one_bounded(RECV_NUM).await;
        let start = SystemTime::now();
        hdl.start().await;
        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {RECV_NUM}: {:>10} [ops/s]",
            ((MAX_COUNT / 10 * RECV_NUM) as f64 / sec) as usize
        );
    });

    println!("tokio (tokio::sync::mpsc::channel)");
    runtime.block_on(async {
        let mut hdl = tokio_bench::new_many_to_one_bounded(RECV_NUM).await;
        let start = SystemTime::now();
        hdl.start().await;
        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!(
            "n = {RECV_NUM}: {:>10} [ops/s]",
            ((MAX_COUNT / 10 * RECV_NUM) as f64 / sec) as usize
        );
    });
}

fn run_one_to_one(f: fn(usize) -> std_thread::OneToOne) {
    for i in 1..=8 {
        let mut h = f(i);
        let start = SystemTime::now();

        h.start();

        let dur = start.elapsed().unwrap();
        let sec = dur.as_secs_f64();
        println!("n = {i}: {:>10} [ops/s]", (MAX_COUNT as f64 / sec) as usize);
    }
}

fn run_many_to_one(f: fn(usize) -> std_thread::ManyToOne) {
    let mut h = f(RECV_NUM);
    let start = SystemTime::now();

    h.start();

    let dur = start.elapsed().unwrap();
    let sec = dur.as_secs_f64();
    println!(
        "n = {RECV_NUM}: {:>10} [ops/s]",
        ((MAX_COUNT / 10 * RECV_NUM) as f64 / sec) as usize
    );
}
