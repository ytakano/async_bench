use async_barrier::Barrier;
use async_std::{
    channel::{self, Receiver, Sender},
    task::JoinHandle,
};
use std::sync::Arc;

pub struct OneToOneAsync {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl OneToOneAsync {
    pub async fn start(&mut self) {
        self.barrier.wait().await;
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.await;
        }
    }

    pub fn new(n: usize, f: fn() -> (Sender<usize>, Receiver<usize>)) -> Self {
        let mut v = Vec::new();
        let barrier = async_barrier::Barrier::new(2 * n + 1);
        let barrier = Arc::new(barrier);

        for _ in 0..n {
            let (tx, rx) = f();

            // Create a sender.
            let bar = barrier.clone();
            let th = async_std::task::spawn(async move {
                bar.wait().await;
                for _ in 0..crate::MAX_COUNT {
                    tx.send(1).await.unwrap();
                }
                0
            });
            v.push(th);

            // Create a receiver.
            let bar = barrier.clone();
            let th = async_std::task::spawn(async move {
                bar.wait().await;
                let mut cnt = 0;
                for _ in 0..crate::MAX_COUNT {
                    let n = rx.recv().await.unwrap();
                    cnt += n;
                }
                cnt
            });
            v.push(th);
        }

        OneToOneAsync {
            handler: v,
            barrier,
        }
    }
}

pub async fn new_one_to_one_unbounded(n: usize) -> OneToOneAsync {
    OneToOneAsync::new(n, channel::unbounded)
}

pub async fn new_one_to_one_bounded(n: usize) -> OneToOneAsync {
    fn mkch() -> (Sender<usize>, Receiver<usize>) {
        channel::bounded(1024)
    }
    OneToOneAsync::new(n, mkch)
}

pub struct ManyToOneAsync {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl ManyToOneAsync {
    pub async fn start(&mut self) {
        self.barrier.wait().await;
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.await;
        }
    }

    pub fn new(n: usize, f: fn() -> (Sender<usize>, Receiver<usize>)) -> Self {
        let mut v = Vec::new();
        let barrier = async_barrier::Barrier::new(n + 2);
        let barrier = Arc::new(barrier);
        let (tx, rx) = f();
        let max_count = crate::MAX_COUNT / 10;

        // Create a receiver.
        let bar = barrier.clone();
        let th = async_std::task::spawn(async move {
            bar.wait().await;
            let mut cnt = 0;
            for _ in 0..(max_count * n) {
                let n = rx.recv().await.unwrap();
                cnt += n;
            }
            cnt
        });
        v.push(th);

        for _ in 0..n {
            // Create a sender.
            let bar = barrier.clone();
            let ch = tx.clone();
            let th = async_std::task::spawn(async move {
                bar.wait().await;
                for _ in 0..max_count {
                    ch.send(1).await.unwrap();
                }
                0
            });
            v.push(th);
        }

        ManyToOneAsync {
            handler: v,
            barrier,
        }
    }
}

pub async fn new_many_to_one_bounded(n: usize) -> ManyToOneAsync {
    fn mkch() -> (Sender<usize>, Receiver<usize>) {
        channel::bounded(1024)
    }
    ManyToOneAsync::new(n, mkch)
}
