use async_barrier::Barrier;
use std::sync::Arc;
use tokio::{sync::mpsc, task::JoinHandle};

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
