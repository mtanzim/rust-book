#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn build_rect(width:u32, height:u32) -> Rectangle {
    Rectangle { width, height }
}

fn main() {
    let rect1 = build_rect(30, 50);

    println!("The rectangle is: {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    )
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
