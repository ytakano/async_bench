use async_barrier::Barrier;
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};

pub struct OneToOneTokio {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl OneToOneTokio {
    pub async fn start(&mut self) {
        self.barrier.wait().await;
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.await.unwrap();
        }
    }

    pub fn new_unbounded(n: usize) -> Self {
        let mut v = Vec::new();
        let barrier = async_barrier::Barrier::new(2 * n + 1);
        let barrier = Arc::new(barrier);

        for _ in 0..n {
            let (tx, mut rx) = mpsc::unbounded_channel();

            // Create a sender.
            let bar = barrier.clone();
            let th = tokio::task::spawn(async move {
                bar.wait().await;
                for n in 0..crate::MAX_COUNT {
                    tx.send(1).unwrap();
                    if n & 0xff == 0 {
                        tokio::task::yield_now().await;
                    }
                }
                0
            });
            v.push(th);

            // Create a receiver.
            let bar = barrier.clone();
            let th = tokio::task::spawn(async move {
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

        Self {
            handler: v,
            barrier,
        }
    }

    pub fn new_bounded(n: usize) -> Self {
        let mut v = Vec::new();
        let barrier = async_barrier::Barrier::new(2 * n + 1);
        let barrier = Arc::new(barrier);

        for _ in 0..n {
            let (tx, mut rx) = mpsc::channel(1024);

            // Create a sender.
            let bar = barrier.clone();
            let th = tokio::task::spawn(async move {
                bar.wait().await;
                for n in 0..crate::MAX_COUNT {
                    tx.send(1).await.unwrap();
                    if n & 0xff == 0 {
                        tokio::task::yield_now().await;
                    }
                }
                0
            });
            v.push(th);

            // Create a receiver.
            let bar = barrier.clone();
            let th = tokio::task::spawn(async move {
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

        Self {
            handler: v,
            barrier,
        }
    }
}

pub async fn new_one_to_one_unbounded(n: usize) -> OneToOneTokio {
    OneToOneTokio::new_unbounded(n)
}

pub async fn new_one_to_one_bounded(n: usize) -> OneToOneTokio {
    OneToOneTokio::new_bounded(n)
}

pub struct ManyToOneTokio {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl ManyToOneTokio {
    pub async fn start(&mut self) {
        self.barrier.wait().await;
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.await.unwrap();
        }
    }

    pub fn new_bounded(n: usize) -> Self {
        let mut v = Vec::new();
        let barrier = async_barrier::Barrier::new(n + 2);
        let barrier = Arc::new(barrier);
        let (tx, mut rx) = mpsc::channel(1024);

        let max_count = crate::MAX_COUNT / 10;

        // Create a receiver.
        let bar = barrier.clone();
        let th = tokio::task::spawn(async move {
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
            let th = tokio::task::spawn(async move {
                bar.wait().await;
                for n in 0..max_count {
                    ch.send(1).await.unwrap();
                    if n & 0xff == 0 {
                        tokio::task::yield_now().await;
                    }
                }
                0
            });
            v.push(th);
        }

        Self {
            handler: v,
            barrier,
        }
    }
}

pub async fn new_many_to_one_bounded(n: usize) -> ManyToOneTokio {
    ManyToOneTokio::new_bounded(n)
}

pub struct MutexBench {
    handler: Vec<JoinHandle<()>>,
    barrier: Arc<Barrier>,
}

impl MutexBench {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::new();
        let barrier = Arc::new(Barrier::new(n + 1));
        let shared = Arc::new(Mutex::new(0));

        for _ in 0..n {
            let bar = barrier.clone();
            let n = shared.clone();
            let th = tokio::task::spawn(async move {
                bar.wait().await;
                for _ in 0..crate::MAX_COUNT {
                    let mut guard = n.lock().await;
                    *guard += 1;
                }
            });
            v.push(th);
        }

        Self {
            handler: v,
            barrier,
        }
    }

    pub async fn start(&mut self) {
        self.barrier.wait().await;
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.await.unwrap();
        }
    }
}
