use std::{fmt::format, slice::from_ref};

trait Print {
    fn print(&self) {
        println!("Cannot print for some reason?!??");
    }
}
fn print_with_hello<T: Print>(variable: &T) {
    println!("Hello, see the printed item under:");
    variable.print()
}

#[derive(Debug)]
struct Dog {
    name: Option<String>,
    age: Option<u8>,
}
impl Print for Dog {
    fn print(&self) {
        println!("{:#?}", self)
    }
}

#[derive(Debug)]
struct Cat {
    name: Option<String>,
    age: Option<u8>,
}
impl Print for Cat {}

fn main() {
    let dog1 = Dog {
        name: Some(String::from("doglol")),
        age: Some(100),
    };
    let dog2 = Dog {
        name: Some(String::from("dog2lol")),
        age: Some(20),
    };
    let cat1 = Cat {
        name: Some(String::from("catlol")),
        age: Some(100),
    };

    dog1.print();
    print_with_hello(&dog2);
    cat1.print();
}
