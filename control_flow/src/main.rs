fn main() {
    let x: i32 = 32;
    if x == 10 {
        println!("Equals 10");
    } else if x > 10 {
        println!("Higher than 10");
    } else {
        println!("Lower than 10");
    }

    //if's can be used on variables
    let y: i32 = if x > 10 { x / 2 } else { x * 2 };
    println!("The value of y is {y}");

    //loop
    let mut i: i32 = 0;
    loop {
        if i == 5 {
            println!("{i} loop");
            println!("stop");
            break;
        } else {
            println!("{i} loop");
        }
        i += 1;
    }

    //you can return a value by putting an value in front of the break
    i = 0;
    let result: i32 = loop {
        i += 1;
        if i == 20 {
            break i / 2;
        }
    };
    println!("The value of result is {result}");

    //you can name nested loops by giving it a 'name
    i = 0;
    let x: i32 = 'first: loop {
        loop {
            if i >= 20 {
                break 'first i * 3;
            }
            i += 1;
        }
    };
    println!("The value of x is {x}");

    //while loop
    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    let arr: [&str; 4] = ["yellow", "red", "blue", "green"];

    for color in arr {
        println!("Color {color}");
    }

    for number in 1..10 {
        println!("{number}");
    }
}
