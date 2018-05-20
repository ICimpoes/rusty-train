use std::collections::HashMap;


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


    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let score = scores.get("Blue");
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let zipped = teams.iter().zip(initial_scores.iter());
    let scores: HashMap<_, _> = zipped.collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name -> value used here after mov

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    people_dep();

    println!("{}", mean(&vec![1, 2, 3, 4]));
    println!("{}", median(&mut vec![1, 2, 3, 4]));
}

fn mean(l: &Vec<usize>) -> f32 {
    let mut t: f32 = 0.0;
    for i in l {
        t += (*i as f32);
    }

    t / (l.len() as f32)
}

fn median(l: &mut Vec<usize>) -> usize {
    l.sort();
    l[l.len() / 2]
}

fn people_dep() {
    let mut p:HashMap<String, Vec<String>> = HashMap::new();

    add(&mut p, String::from("X"), String::from("Tod"));
    add(&mut p, String::from("X"), String::from("Ted"));
    add(&mut p, String::from("S"), String::from("Ben"));
    add(&mut p, String::from("S"), String::from("Ann"));

    get_and_sort(&mut p, &String::from("X"));
    get_and_sort(&mut p, &String::from("S"));
}

fn add(people: &mut HashMap<String, Vec<String>>, d: String, p: String) {
    let v = people.entry(d).or_insert(vec![]);
    v.push(p);
}

fn get_and_sort(people: &mut HashMap<String, Vec<String>>, dep: &String) {
    if let Some(v) = people.get_mut(dep) {
        v.sort();
        println!("{:?}", v);
    } else {
        println!("not found");
    }
}