fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    print_summary(&tweet);
    print_summary(&TrafficLight::Red);


    fn print_summary<S: Summary>(s: &S) {
        println!("summary {}", s.summarize());
    }
    tweet.print();

    let article = NewsArticle{
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    article.print()

}

pub trait Summary {
    fn summarize(&self) -> String;
    fn print(&self) {
        println!("summary {}", self.summarize());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn print(&self) {
        println!("article summary {}", self.summarize());
    }

}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

extern crate communicator;

use communicator::*;

impl Summary for TrafficLight {
    fn summarize(&self) -> String {
        match *self {
            TrafficLight::Red => String::from("RED"),
            TrafficLight::Yellow=> String::from("YELLOW"),
            TrafficLight::Green=> String::from("GREEN")
        }
    }
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: std::fmt::Display + Clone,
          U: Clone + std::fmt::Debug {
    1
}