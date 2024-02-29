use garden::vegetables::Carrot;
pub mod garden;

fn main() {
    let carrot1 = Carrot { weight: 30 };
    carrot1.print_carrot();
}

