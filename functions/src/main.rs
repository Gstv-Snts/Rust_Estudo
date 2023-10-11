fn main() {
    println!("Hello, world!");
    another_function();
    sum(23230, -2323);
    //variables can have expressions as long it is an declaration
    let x: i32 = {
        let y: i32 = 30;
        y * 2
    };
    println!("The value f x is {x}");
    let sub: i32 = subtract(23, -40);
    println!("The value f sub is {sub}");

    let mult: i32 = multiply(23, -40);
    println!("The value f mult is {mult}");
}

fn another_function() {
    println!("Another, hello!");
}

fn sum(first_number: i32, second_number: i32) {
    let sum: i32 = first_number + second_number;
    println!("Sum: {sum}");
}

//a function is an "declaration" and a variable is an "declaration"

//you can return using an operation without ;
fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

//or you can return using return and ;
fn multiply(first_number: i32, second_number: i32) -> i32 {
    return first_number * second_number;
}
