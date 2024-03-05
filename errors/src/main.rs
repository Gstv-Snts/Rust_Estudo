use core::panic;
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    println!("{:#?}",read_file_content("text.txt"));
    let vec = vec![1, 2, 3, 5, 6, 8];
    //causes panic vec[10];
    let file_content = File::open("text.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("text.txt")
                .unwrap_or_else(|error| panic!("Problem creating an file: {:#?}", error))
        } else {
            panic!("Problem opening an file: {:#?}", error);
        }
    });
    let file_content = File::open("text.txt").unwrap();
    let file_content = File::open("text.txt").expect("Problem opening file text.txt");
    println!("File content: {:#?}", file_content);
}

fn read_file_content(path: &str) -> Result<String, io::Error> {
    Ok(fs::read_to_string(path)?)
}
