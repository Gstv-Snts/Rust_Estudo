struct Person {
    name: String,
    age: u8,
    email: String,
    id: u32,
}
fn main() {
    let mut p1 = Person {
        name: String::from("John"),
        age: 30,
        email: String::from("john123@gmail.com"),
        id: 32030230,
    };
    p1.age = 31;
    println!(
        "The value of p1 is name: {}, age: {}, email: {}, id: {}",
        p1.name, p1.age, p1.email, p1.id
    );

    let p2 = build_person(
        String::from("test"),
        23,
        String::from("test123@gmail.com"),
        230209424,
    );
    println!(
        "The value of p2 is name: {}, age: {}, email: {}, id: {}",
        p2.name, p2.age, p2.email, p2.id
    );

    let p3 = Person {
        email: String::from("test321@gmail.com"),
        ..p2
    };
    println!(
        "The value of p3 is name: {}, age: {}, email: {}, id: {}",
        p3.name, p3.age, p3.email, p3.id
    );
    //tuple struct
    struct Values(u32, String, bool);
    let v1 = Values(30, String::from("text"), false);
    println!("The value of v1.0 is {}", v1.0);
    println!("The value of v1.1 is {}", v1.1);
    println!("The value of v1.2 is {}", v1.2);

    //unit-like struct
    //struct Unit;
}

fn build_person(name: String, age: u8, email: String, id: u32) -> Person {
    Person {
        name,
        age,
        email,
        id,
    }
}
