fn route(ip_type: IpAddrKind) { }

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    //println!("{:?}", four);


    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");


    let x = some_string.and_then(|x: &str| Some("sf"));
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}, {:?}", some_number, some_string, absent_number, x);

    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }


    let some_u8_value: Option<i32> = None;

    if let Some(x) = some_u8_value {
        println!("some: {}", x);
    } else {
        println!("none");
    }


    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }


}


enum  IpAddr {
    V4(String),
    V6(String),
}

enum  IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
/*

enum Option<T> {
    Some(T),
    None,
}*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
