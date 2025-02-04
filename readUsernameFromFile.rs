use std::io;
use std::io::error;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?;
    let mut s = String::From("");
    f.read_to_string(&mut s)?;
    Ok(s)
}

// the code coudl be shortenend by chaining method calls together
fn read_name_from_file() -> Result<String, io::Error> {
    let mut s = String::From("");
    let f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// the ? operator can only be used in function that returns Result