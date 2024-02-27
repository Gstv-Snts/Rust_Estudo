#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self{
        Self{
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn name(&self) {
        println!("{:?}", self)
    }
    fn width(&self) -> u32 {
        self.width * 2
    }
    fn can_hold(&self, rec2: Rectangle) -> bool {
        if self.height == rec2.height && self.width == rec2.width {
            return true;
        }
        false
    }
}
fn main() {
    let mut rec = Rectangle {
        height: dbg!(30 * 2),
        width: 20,
    };
    rec.width = rec.width();
    println!("The rectangle is {:#?}", rec);
    println!("The area of the rectangle is {}", rec.area());
    rec.name();
    let rec2 = Rectangle {
        height: 50,
        width: 40,
    };
    println!("The rectangle 2 is {:#?}",rec2);
    if rec.can_hold(rec2) {
        println!("Both rectangles are the same!")
    } else {
        println!("The rectangles are different")
    };
    let rec3: Rectangle = Rectangle::square(30);
    println!("{:#?}",rec3)
}
