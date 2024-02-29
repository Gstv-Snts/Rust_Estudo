#[derive(Debug)]
pub struct Carrot {
    pub weight: i32 
}
impl Carrot {
   pub fn print_carrot(&self){
        println!("The carrot is {} pounds!", self.weight)
   } 
}
