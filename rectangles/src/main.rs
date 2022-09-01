#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size:u32) -> Self {
        Self {
            width:size,
            height: size,
        }
    }
}

fn build_rect(width:u32, height:u32) -> Rectangle {
    Rectangle { width, height }
}

fn main() {
    let rect1 = build_rect(30, 50);
    let rect2 = build_rect(35, 55);
    let square1 = Rectangle::square(10);

    println!("The rectangle is: {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("The rectangle {:?} can hold {:?}: {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("The rectangle {:?} can hold {:?}: {}", rect2, rect1, rect2.can_hold(&rect1));
    println!("The rectangle {:?} is a square", square1);

}

