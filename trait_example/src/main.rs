pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Blurb {
    pub commenter: String,
    pub text: String,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}, ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.content, self.username)
    }
}

impl Summary for Blurb {}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let blurb = Blurb {
        commenter: String::from("Tanzim"),
        text: String::from("Hello"),
    };

    notify(&tweet);
    notify(&blurb);
}
