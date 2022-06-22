use std::time::SystemTime;
use tokio::runtime::Runtime as TokioRuntime;

mod async_std_bench;
mod std_thread;
mod thread_crossbeam;
mod thread_flume;
mod tokio_bench;

const MAX_COUNT: usize = 10000000;

fn main() {
    println!("std::thread (std::sync::mpsc::channel)");
    run_one_to_one(std_thread::new_channel);

    println!("std::thread (std::sync::mpsc::sync_channel)");
    run_one_to_one(std_thread::new_sync_channel);

    println!("std::thread (flume::unbounded)");
    run_one_to_one(thread_flume::new_unbounded);

    println!("std::thread (flume::bounded)");
    run_one_to_one(thread_flume::new_bounded);

    println!("std::thread (crossbeam::channel::unbounded)");
    run_one_to_one(thread_crossbeam::new_unbounded);

    println!("std::thread (crossbeam::channel::bounded)");
    run_one_to_one(thread_crossbeam::new_bounded);

    println!("async_std (async_std::channel::unbounded)");
    for i in 1..=8 {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_unbounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
        });
    }

    println!("async_std (async_std::channel::bounded)");
    for i in 1..=8 {
        async_std::task::block_on(async {
            let mut hdl = async_std_bench::new_one_to_one_bounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
        });
    }

    let runtime = TokioRuntime::new().unwrap();

    println!("tokio (tokio::sync::mpsc::unbounded_channel)");
    for i in 1..=8 {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_unbounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
        });
    }

    println!("tokio (tokio::sync::mpsc::channel)");
    for i in 1..=8 {
        runtime.block_on(async {
            let mut hdl = tokio_bench::new_one_to_one_bounded(i).await;
            let start = SystemTime::now();
            hdl.start().await;
            println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
        });
    }
}

fn run_one_to_one(f: fn(usize) -> std_thread::OneToOne) {
    for i in 1..=8 {
        let mut h = f(i);
        let start = SystemTime::now();

        h.start();

        println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
    }
}
