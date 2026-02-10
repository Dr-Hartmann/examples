use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn read_username_from_file() -> String {
    ret_username().unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });
            ret_username().expect("hello.txt should be included in this project")
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

fn ret_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Или так проще:
    // use std::fs;
    // fs::read_to_string("hello.txt")
}
