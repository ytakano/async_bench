use std::time::SystemTime;

mod std_thread;
mod thread_crossbeam;
mod thread_flume;

const MAX_COUNT: usize = 100000000;

fn main() {
    println!("std::thread (std::sync::mpsc)");

    for i in 1..=8 {
        let mut h = std_thread::OneToOne::new(i);
        let start = SystemTime::now();

        h.start();

        println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
    }

    println!("std::thread (flume::unbounded)");

    for i in 1..=8 {
        let mut h = thread_flume::OneToOne::new(i);
        let start = SystemTime::now();

        h.start();

        println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
    }

    println!("std::thread (crossbeam::unbounded)");

    for i in 1..=8 {
        let mut h = thread_crossbeam::OneToOne::new(i);
        let start = SystemTime::now();

        h.start();

        println!("n = {i}: {} [ms]", start.elapsed().unwrap().as_millis());
    }

    println!("Hello, world!");
}
