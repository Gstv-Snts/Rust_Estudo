fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of _guess is: {_guess}");
    //i8 = 128 - 127
    let signed8bit: i8 = 127;
    println!("The value of signed8bit is: {signed8bit}");
    //u8 = 0 - 255
    let unasigned8bit: u8 = 255;
    println!("The value of unasigned8bit is: {unasigned8bit}");

    let unasigned16bit: u16 = 65535;
    println!("The value of unasigned16bit is: {unasigned16bit}");

    let unasigned32bit: u32 = 4294967295;
    println!("The value of unasigned32bit is: {unasigned32bit}");

    let unasigned64bit: u64 = 9999;
    println!("The value of unasigned64bit is: {unasigned64bit}");

    let unasigned128bit: u128 = 9999;
    println!("The value of unasigned128bit is: {unasigned128bit}");

    //the max value of an usize is define by the the system, 64-bit = u64, 32-bit = u32
    let unasignedusize: usize = 9999;
    println!("The value of unasignedusize is: {unasignedusize}");

    let decimal_number: u32 = 98_222;
    println!("The value of decimal_number is: {decimal_number}");

    let hex_number: u32 = 0xff;
    println!("The value of hex_number is: {hex_number}");

    let octal_number: u32 = 0o77;
    println!("The value of octal_number is: {octal_number}");

    let binary_number: u32 = 0b1111_0000;
    println!("The value of binary_number is: {binary_number}");

    let byte_number: u8 = b'A';
    println!("The value of byte_number is: {byte_number}");

    let x = 2.0; // f64
    println!("The value of x is: {x}");

    let y: f32 = 3.0; // f32
    println!("The value of y is: {y}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    let t = true;
    println!("The value of t is: {t}");

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");

    let c = 'z';
    println!("The value of c is: {c}");

    let z: char = 'ℤ'; // with explicit type annotation
    println!("The value of z is: {z}");

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_first: i32 = tup.0;
    let tup_second: f64 = tup.1;
    let tup_third: u8 = tup.2;
    println!("The value of tup is: {tup_first}, {tup_second}, {tup_third}");
    let (x, y, z) = tup;
    println!("The value of tup is: {x}, {y}, {z}");

    let arr: [i32; 4] = [2, 34, 54, 6];
    let arr_first: i32 = arr[0];
    let arr_second: i32 = arr[1];
    let arr_third: i32 = arr[2];
    let arr_fourth: i32 = arr[3];
    println!("The value of arr is: {arr_first}, {arr_second}, {arr_third}, {arr_fourth}");
}
