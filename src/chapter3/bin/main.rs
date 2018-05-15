fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
//-----------------------------

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev().rev() {
        println!("{}!", number);
    }
//-----------------------------


    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}!", s);
    let s = String::from("hello");
    takes_ownership(s.clone());
    println!("{}!", s);

    let x = 5;

    makes_copy(x);

    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push('a');

    println!("{}", r1);
//-----------------------------

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    let s1 = String::from("hello");

    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);


    let mut s = String::from("aasdas");
    let len = calculate_length2(&s);
    change(&mut s);
    let r1 = &mut s;

    println!("The length of '{}' {}", r1, len);


//-----------------------------

    println!("{}", first_word(&String::from("hell o world")));

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}  {}", hello, world);

    println!("{}", first_word2("hell o world"));

    let mut s = String::from("hello world");

    {
        let word = first_word2(&s);
        println!("{}", word);
//        s.clear(); //compile error
    }

    s.clear();
//-----------------------------
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let mut user1 ...
//    user1.email = String::from("anotheremail@example.com");

    println!("user: {:?}", user1);

    let user2 = User {
//        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user: {:?}", user2);
    //-----------------------------
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
