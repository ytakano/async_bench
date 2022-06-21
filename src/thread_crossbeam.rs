use crossbeam::channel;
use std::{
    sync::{Arc, Barrier},
    thread::JoinHandle,
};

pub struct OneToOne {
    handler: Vec<JoinHandle<u32>>,
    barrier: Arc<Barrier>,
}

impl OneToOne {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::new();

        let barrier = Arc::new(Barrier::new(n * 2 + 1));

        for _ in 0..n {
            let (tx, rx) = channel::unbounded();

            // Create a sender.
            let bar = barrier.clone();
            let th = std::thread::spawn(move || {
                bar.wait();
                for _ in 0..crate::MAX_COUNT {
                    tx.send(1).unwrap();
                }
                0
            });
            v.push(th);

            // Create a receiver.
            let bar = barrier.clone();
            let th = std::thread::spawn(move || {
                bar.wait();
                let mut cnt = 0;
                for _ in 0..crate::MAX_COUNT {
                    let n = rx.recv().unwrap();
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

    pub fn start(&mut self) {
        self.barrier.wait();
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.join().unwrap();
        }
    }
}
