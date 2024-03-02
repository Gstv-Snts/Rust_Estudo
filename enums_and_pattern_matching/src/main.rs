#[derive(Debug)]
enum Vehicle {
    Car(String, u8),
    Motorcycle(String),
}
impl Vehicle {
    fn print(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
enum Dog {
    Name(String),
}
impl Dog {
    fn walk(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum Estado {
    RS,
    RJ,
    SP,
}

#[derive(Debug)]
enum Coin {
    UmCentavo,
    VinteCincoCentavos,
    CinquentaCentavos(Estado),
}
impl Coin {
    fn get_value(&self) -> i8 {
        match self {
            Coin::UmCentavo => {
                println!("Returning 1 cent!");
                1
            }
            Coin::VinteCincoCentavos => 25,
            Coin::CinquentaCentavos(estado) => {
                match estado {
                    Estado::RS => {
                        println!("The 50 centavos coin is red!");
                        50
                    }
                    Estado::RJ => {
                        println!("The 50 centavos coin is blue!");
                        50
                    }
                    Estado::SP => {
                        println!("The 50 centavos coin is yellow!");
                        50
                    }
                };
                50
            }
        }
    }
}

#[derive(Debug)]
struct Dice {
    value: u8,
}
impl Dice {
    fn roll(&mut self, newvalue: u8) {
        match newvalue {
            1 => println!("You die!"),
            20 => println!("You do 40 damage!"),
            _ => println!("Your do {} damage!", newvalue),
        }
        self.value = newvalue;
    }
}

fn main() {
    let car = Vehicle::Car(String::from("corola"), 3);
    car.print();
    let motorcycle = Vehicle::Motorcycle(String::from("honda"));
    motorcycle.print();
    let dog = Dog::Name(String::from("loool"));
    dog.walk();
    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_char = Some('e');
    println!("{:?}", some_char);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);

    let um = Coin::UmCentavo;
    let vinte_cinco = Coin::VinteCincoCentavos;
    println!(
        "The coins are {}, and {}.",
        um.get_value(),
        vinte_cinco.get_value()
    );
    let cinquenta = Coin::CinquentaCentavos(Estado::RJ);
    println!("I have an {} centavos coin", cinquenta.get_value());

    let stuff: String = String::from("soap");
    print_stuff(Some(stuff));
    print_stuff(None);

    let mut dice = Dice { value: 10 };
    dice.roll(1);
    dice.roll(20);
    let mut name = Some(String::from("Gustavo"));
    match &name {
        Some(name) => println!("The name is {}", name),
        _ => (),
    }
    name = None;
    if let Some(name) = &name {
        println!("The name is {}", name);
    } else {
        println!("There is no name");
    }
}

fn print_stuff(stuff: Option<String>) {
    match stuff {
        None => println!("No stuff!"),
        Some(s) => println!("Stuff is {}", s),
    }
}
