const MAX_COUNT: usize = 10000;

pub mod async_std_bench;
pub mod std_thread;
pub mod thread_crossbeam;
pub mod thread_flume;
pub mod tokio_bench;
