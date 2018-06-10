use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);


    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let mut file = File::open(filename).expect("could not open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("could not read file");

    println!("File content:\n {}", contents)
}