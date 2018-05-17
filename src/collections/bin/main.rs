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


}