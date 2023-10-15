fn main() {
    let s: String = String::from(" test");
    let s_len: usize = calculate_len(&s);
    //the function uses the reference instead of taking the ownership
    //this method of using an value without taking ownership is called borrowing
    println!("The value of s is {s}");
    println!("The value of s_len is {s_len}");

    let mut s2: String = String::from("text");
    change_mut(&mut s2); //sends an mutable variable
    println!("The value of s2 is {s2}");
    //you can only have one reference to a mutable variable at a time
    //you have to use the first reference before referencing it on another variable
    //you can have references if they are all in different scopes
    //you also cant reference to an immutable variable and reference to a muttable variable right after, but you can have as many immutable references as you want
    let mut text: String = String::from("text");
    let t1: &String = &text;
    let t2: &String = &text;
    println!("The value of t1 is {t1}");
    println!("The value of t2 is {t2}");

    let t3: &mut String = &mut text;
    println!("The value of t3 is {t3}");

    let dangle = dangle();
    println!("The value of dangle is {dangle}");
}

fn calculate_len(s: &String) -> usize {
    //requires an reference from an string
    //because this function doesnt own the variable s you cant change its value
    //using .push_string()
    s.len()
}

fn change_mut(s: &mut String) {
    //you can change an reference by adding &mut before the variable
    s.push_str(" aditional text");
}

fn dangle() -> String {
    let text: String = String::from("text");
    text
    //you cant return a reference to a variable inside this scope, so to return the "text" variable you should return the variable and not its reference, this way the function also returns the ownership
}
