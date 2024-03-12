use std::thread;

pub fn test() {
    println!(" [ concurrency test ] ");

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();
}