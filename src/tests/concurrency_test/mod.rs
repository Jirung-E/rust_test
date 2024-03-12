use std::sync::mpsc;
use std::thread;

pub fn test() {
    println!(" [ concurrency test ] ");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}