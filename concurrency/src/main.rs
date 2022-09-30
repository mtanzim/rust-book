use std::sync::mpsc;
use std::{thread, time::Duration};

fn threads_with_move_closure() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi, number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("hi {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn messages() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = vec![
            String::from("hello"),
            String::from("from"),
            String::from("my"),
            String::from("thread"),
            String::from("yo"),
        ];
        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000))
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("this"),
            String::from("is"),
            String::from("a"),
            String::from("different"),
            String::from("thread"),
        ];
        for v in val {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000))
        }
    });

    for received in rx {
        println!("Got: {}", received)
    }
}

fn main() {
    println!("Hello, world!");
    threads();
    threads_with_move_closure();
    messages();
}
