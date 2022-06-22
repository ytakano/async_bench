use crate::std_thread::OneToOne;
use flume;

pub fn new_unbounded(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = flume::unbounded();
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}

pub fn new_bounded(n: usize) -> OneToOne {
    fn mkch() -> (Box<dyn Fn(usize) + Send>, Box<dyn Fn() -> usize + Send>) {
        let (tx, rx) = flume::bounded(1024);
        (
            Box::new(move |x| tx.send(x).unwrap()),
            Box::new(move || rx.recv().unwrap()),
        )
    }

    OneToOne::new(n, mkch)
}
