fn main() {
    //muttable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //constant variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    //shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
    //type changing using shadowing
    let spaces = "   ";
    println!("The value of spaces is: '{spaces}'");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    //you cant change the value of an let mut variable
}