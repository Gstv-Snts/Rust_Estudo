fn main() {
    //name is not declared
    let mut name: String = String::from("john");
    //name is declared
    name.push_str(" smith");
    println!("The value of name is {name}");

    //rust invalidates the first variable, so instead of a copy
    //it is called an "move"
    let mut name2: String = name;
    //name doest not exits for now on, if you try to use it you will get an error
    println!("The value of name2 is {name2}");

    name2.push_str(" lollol");
    println!("The value of name2 is {name2}");

    let x: String = String::from("clone");
    //.clone deeply copy the heap data
    let x2: String = x.clone();
    println!("The value of x is {x}");
    println!("The value of x2 is {x2}");

    //there is no deep or shallow copying difference between integers
    let y: i32 = 5;
    let y2: i32 = y;
    println!("The value of y is {y}");
    println!("The value of y2 is {y2}");
    //only types that dont require alocation have the copy trait, for exemple
    //integers, booleans, floatings, chars, and tuples if they ony contain the previous

    let z_string: String = String::from("test"); //string exists here
    print_string(z_string); //string stop existing here because it requires alocation

    let z_int: i32 = 20; //int exists here
    print_int(z_int); //it keeps existing because it is an int
    println!("The value of z_int is {z_int}");

    let ro_string = returns_ownership();
    println!("The value of ro_string is {ro_string}");

    let mut toagib = String::from("new text");
    toagib = takes_ownership_and_gives_it_back(toagib);
    println!("The value of toagib is {toagib}");

    let s1: String = String::from("test");
    let (s2, len): (String, usize) = calculate_length(s1);
    println!("The value of s2 is {s2}");
    println!("The value of len is {len}");
}

fn print_int(n: i32) {
    //takes an copy of the integer
    println!("The value of int is {n}");
}

fn print_string(s: String) {
    //takes the ownership of the string
    println!("The value of string is {s}");
}

fn returns_ownership() -> String {
    let new_string: String = String::from("test");
    new_string
}

fn takes_ownership_and_gives_it_back(s: String) -> String {
    s + " more text"
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s, length)
}
