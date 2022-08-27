fn fib(n: i64) -> i64 {
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    if n < 0 {
        return -1;
    } else if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    } else {
        for _i in 2..=n + 1 {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}

fn main() {
    println!("Hello, world!");
    for i in 0..=50 {
        println!("{}", fib(i));
    }
}
