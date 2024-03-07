fn main() {
    let x = 5;
    let r;
    r = &x;
    println!("r: {}", r);
    println!("Biggest number 3 - 10: {}", biggest_number(&3, &10));
    println!("Biggest number 7 - 5: {}", biggest_number(&7, &5));
}
fn biggest_number<'a>(num1: &'a u8, num2: &'a u8) -> &'a u8 {
    match num1 >= num2 {
        true => &num1,
        false => &num2,
    }
}
