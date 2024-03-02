mod front_of_the_house;
pub use crate::front_of_the_house::hosting;

mod back_of_the_house;
pub use crate::back_of_the_house::{Appetizer, Breakfast};

use rand::*;
use std::collections::{HashMap as hm, LinkedList as ll};

pub fn eat_at_the_restaurant() {
    let salad = Appetizer::Salad;
    let soup = Appetizer::Soup;
    let mut meal = Breakfast::summer("pão de centeio");
    meal.toast = String::from("pão branco");
    println!("{:#?}", meal);
    println!("{:#?}", salad);
    println!("{:#?}", soup);
    let mut map = hm::new();
    map.insert(1, "banana");
    let mut ll = ll::new();
    ll.push_front("alo");
    println!("The random number is {}", thread_rng().gen_range(0..=100));
}
