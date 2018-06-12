use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("could not parse arguments: {}", err);
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    let mut file = File::open(config.filename).expect("could not open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("could not read file");

    println!("File content:\n {}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
