use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let expensive_closure = |num| {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let res = expensive_closure(2);
    println!("{res}")
}
