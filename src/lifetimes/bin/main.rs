fn main() {
    let string1 = String::from("abcd");
    let result2;
    {
        let s: &'static str = "asdsa";
        let string2 = String::from("xyz");
        let result;

        result = longest(string1.as_str(), string2.as_str());
        result2 = longest2(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
        println!("The longest string is {}", result2);
    }
//    println!("The longest string is {}", result);
    println!("The longest string is {}", result2);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let p;
    {
        let i = ImportantExcerpt { part: first_sentence, a: String::from("asd") };
        p = i.part();
//        a = i.a();
    }

    println!("part {}", p);
//    println!("a {}", a);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
    a: String,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn part(&self) -> &'a str {
        self.part
    }
    fn a(&self) -> &str {
        self.a.as_str()
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}