#[derive(Debug)]
struct Rectangle<T, U> {
    x: T,
    y: U,
}

fn largest_in_the_vector<T>(things: &[T]) -> &T {
    let mut biggest_thing = &things[0];
    for item in things {
        if item > biggest_thing {
            biggest_thing = item;
        }
    }
    biggest_thing
}
fn main() {
    let nums = vec![10, 30, 2, 600, 23, 103, 400];
    println!("{}", largest_in_the_vector(&nums));
    let integer_rectangle: Rectangle<u32, u32> = Rectangle { x: 30, y: 20 };
    let float_rectangle: Rectangle<f32, f32> = Rectangle { x: 30.0, y: 20.0 };
    let float_integer_rectangle: Rectangle<f32, u32> = Rectangle { x: 30.0, y: 20 };
}
