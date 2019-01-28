use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    test0();
}

fn test0() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn test1() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
