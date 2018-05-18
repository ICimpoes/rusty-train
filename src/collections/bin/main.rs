fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);


    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v);

    } // <- v goes out of scope and is freed here

    println!("{:?}", v);
    println!("{:?}", v[1]);

    println!("{:?}", v.get(4));

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    // strings
    let mut s = String::new();
    println!("{:?}", s);

    let s = "initial contents".to_string();
    println!("{:?}", s);
    let s = String::from("initial");
    println!("{:?}", s);

    let mut s = String::from("foo");
    s.push(' ');
    s.push_str("bar");
    println!("{:?}", s);



    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    s1.insert_str(3, " ");
    println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("s1 is {}", s1); s1 is moved:
    //    fn add(mut self, other: &str) -> String
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);


    let s1 = String::from("hello");
    //let h = s1[0];
    let len = String::from("Hola").len();
    println!("{}", len);
    let len = String::from("Здра").len();
    println!("{}", len);

//    let h = &"hello"[0];

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    //let s = &hello[0..1]; 3, 5,.. Panics
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }



}