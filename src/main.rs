use std::sync::mpsc::{channel, RecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(0).unwrap();
        thread::sleep(Duration::from_secs(10));
    });
    let _: Result<u8, RecvError> = rx.recv();
    let _: Result<u8, RecvError> = rx.recv();
}
