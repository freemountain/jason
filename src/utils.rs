use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let r = File::open(path);

    if r.is_err() {
        Err(r.unwrap_err())
    } else {
        let mut f = r.unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).and(Ok(buffer))
    }
}

pub fn read_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();

    let result = io::stdin().read_to_string(&mut buffer);

    match result {
        Err(e) => Err(e),
        Ok(_) => Ok(buffer),
    }
}

pub fn split(input: &str) -> Vec<String> {
    let mut vec = Vec::new();

    for part in input.split(',') {
        vec.push(String::from(part));
    }

    vec
}
