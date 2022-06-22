use std::{
    sync::{mpsc, Arc, Barrier},
    thread::JoinHandle,
};

pub struct OneToOne {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl OneToOne {
    pub fn new(
        n: usize,
        mkch: fn() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>),
    ) -> Self {
        let mut v = Vec::new();

        let barrier = Arc::new(Barrier::new(n * 2 + 1));

        for _ in 0..n {
            let (tx, rx) = mkch();

            // Create a sender.
            let bar = barrier.clone();
            let th = std::thread::spawn(move || {
                bar.wait();
                for _ in 0..crate::MAX_COUNT {
                    tx(1);
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
                    let n = rx();
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

pub fn new_channel(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = mpsc::channel();
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub fn new_sync_channel(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = mpsc::sync_channel(1024);
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}
