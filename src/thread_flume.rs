use crate::std_thread::{ManyToOne, OneToOne};
use flume;

pub fn new_one_to_one_unbounded(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = flume::unbounded();
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub fn new_one_to_one_bounded(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = flume::bounded(1024);
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub fn new_many_to_one_bounded(n: usize) -> ManyToOne {
    let (tx, rx) = flume::bounded(1024);
    let mut v = Vec::<Box<dyn Fn(usize) + Send>>::new();

    for _ in 0..n {
        let ch = tx.clone();
        v.push(Box::new(move |msg| {
            ch.send(msg).unwrap();
        }));
    }

    ManyToOne::new(v, Box::new(move || rx.recv().unwrap()))
}
