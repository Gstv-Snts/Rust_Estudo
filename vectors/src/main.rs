use std::ops::Index;

fn main() {
    let mut vec: Vec<u32> = Vec::new();
    vec.push(30);
    vec.push(40);
    vec.push(50);
    vec.push(60);
    println!("vector:{:?}", vec);
    let vec2 = vec![3, 4, 5, 6, 7];
    println!("vector2:{:?}", vec2);
    for i in &vec2 {
        println!("vec2: {}", i);
    }
    #[derive(Debug)]
    enum ManyTypes {
        Int(u32),
        String(String),
        Boolean(bool),
    }
    let vec3 = vec![
        ManyTypes::Int(30),
        ManyTypes::String(String::from("hello")),
        ManyTypes::Boolean(false),
    ];
}
