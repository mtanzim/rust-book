use std::sync::{mpsc, Mutex, Arc};
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

fn mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6
    }
    println!("m = {:?}", m)
}

fn mutex_with_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}

fn main() {
    println!("Hello, world!");
    threads();
    threads_with_move_closure();
    messages();
    mutex();
    mutex_with_threads();
}
