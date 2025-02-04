use std::fs::File;
use std::io::ErrorKind;

fn main () {
    let f = File::open("git.txt");
    let f = match f  {
        Ok(file) => file,
        Error(ref error) if error.kind() == ErrorKind::NotFound {
            match file::create("git-replica.txt") {
                Ok(fc) => fc,
                Err(err) => {
                    panic!("can't create file! check if you have necessary permissions to do so. {:?}", err)
                },
            }
        },
        Error(err) => {
            panic!("something went wrong while opening file", err);
        },
    };
}

// you could as well use let f = File::open("hello.txt").expect("error opening file") to handle error case
// and if you are sure that f is available then you could as well use
// let f = File::open("hello.txt").unwrap();