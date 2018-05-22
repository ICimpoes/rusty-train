fn main() {
    //panic!("crash and burn");


    let v = vec![1, 2, 3];

    // v[99]; panics


    println!("{:?}", read_username_from_file3());

    openFile();
}

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn openFile() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => {
            println!("Success: {:?}", file.metadata())
        }
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => {
                    println!("file created {:?}", fc.metadata())
                }
                Err(e) => {
                    println!("could not create file {:?}", e)
                }
            }
        },
        Err(e) => {
            println!("could not open file {:?}", e)
        }
    };
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}