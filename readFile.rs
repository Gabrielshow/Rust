use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::Open("hello.txt");

    let f = match f {
        Ok(f) => f,
        Err(err) => return Err(err),
    }

    let mut s = String::From("");

    match f.read_to_string(&mut s) {
        Ok(_) -> s,
        Err(err) -> err,
    }
}