use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[allow(dead_code)]
#[allow(unused_variables)]
fn short_read_username_from_file() -> Result<String, io::Error> {
    // on Err(), `?` syntax returns as if `return` were called in from this function.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn long_read_username_from_file() -> Result<String, io::Error> {
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

#[allow(dead_code)]
#[allow(unused_variables)]
fn nesting_error_matches() {
    // nesting matches
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create hello.txt, but failed: {:?}", e)
                }
            },
            other => panic!("other error: {:?}", other),
        },
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn panicking() {
    panic!("crash and burn");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn unwrapping() {
    let f = File::open("hello.txt").unwrap();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn expecting() {
    let f = File::open("hello.txt").expect("some error happened");
}

fn main() {
    match short_read_username_from_file() {
        Ok(_) => println!("it works"),
        Err(e) => panic!("didn't work: {}", e),
    }
}
