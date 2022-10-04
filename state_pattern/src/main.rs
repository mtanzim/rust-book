use state_pattern::{functional, Post};

fn main() {
    // OOP State pattern
    let mut post = Post::new();

    post.add_text("I ate a sandwich");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a sandwich", post.content());

    // FP type based State Pattern
    let mut post_fp = functional::Post::new();
    post_fp.add_text("I ate an apple");
    assert_eq!("", post_fp.content());
    let post_fp = post_fp.request_review();
    assert_eq!("", post_fp.content());
    let post_fp = post_fp.approve();
    assert_eq!("I ate an apple", post_fp.content());
}
