use futures::executor::block_on;


async fn learn_song() {
    println!("learning song")
}


async fn sing_song() {
    println!("singing")
}


async fn dancing() {
    println!("dancing")
}



async fn hello_future() {
    println!("Hello, future")
}

fn main() {
    let future = hello_future();
    block_on(future);
}
