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

pub fn new_one_to_one_channel(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = mpsc::channel();
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub fn new_one_to_one_sync_channel(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = mpsc::sync_channel(1024);
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub struct ManyToOne {
    handler: Vec<JoinHandle<usize>>,
    barrier: Arc<Barrier>,
}

impl ManyToOne {
    pub fn new(tx: Vec<Box<dyn Fn(usize) + Send>>, rx: Box<dyn Fn() -> usize + Send>) -> Self {
        let mut v = Vec::new();
        let barrier = Arc::new(Barrier::new(tx.len() + 2));

        let max_count = crate::MAX_COUNT / 10;
        let tx_len = tx.len();

        // Create a receiver.
        let bar = barrier.clone();
        let th = std::thread::spawn(move || {
            bar.wait();
            let mut cnt = 0;
            for _ in 0..(max_count * tx_len) {
                let n = rx();
                cnt += n;
            }
            cnt
        });
        v.push(th);

        for ch in tx {
            // Create a sender.
            let bar = barrier.clone();
            let th = std::thread::spawn(move || {
                bar.wait();
                for _ in 0..max_count {
                    ch(1);
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

    pub fn start(&mut self) {
        self.barrier.wait();
        let v = std::mem::take(&mut self.handler);
        for th in v {
            th.join().unwrap();
        }
    }
}

pub fn new_many_to_one_sync_channel(n: usize) -> ManyToOne {
    let (tx, rx) = mpsc::sync_channel(1024);
    let mut v = Vec::<Box<dyn Fn(usize) + Send>>::new();

    for _ in 0..n {
        let ch = tx.clone();
        v.push(Box::new(move |msg| {
            ch.send(msg).unwrap();
        }));
    }

    ManyToOne::new(v, Box::new(move || rx.recv().unwrap()))
}
