// src/main.rs

mod menu;
mod dishes;
mod kitchen;


use dishes::{Salad, Flambe};
use kitchen::{Kitchen};

fn main() {
    let mut my_kitchen = Kitchen::new();

    // 1. Create distinct dishes for every order
    let garden_salad_1 = Salad::new("Garden Salad 1", 8.50, 1);
    let garden_salad_2 = Salad::new("Garden Salad 2", 8.50, 1); // A brand new salad!
    let caesar_salad = Salad::new("Caesar Salad", 10.0, 3); 
    let cherry_flambe = Flambe::new("Cherry Flambe", 14.0, 2);
    
    // 2. Move them into the kitchen (Each variable is only used ONCE)
    my_kitchen.add_order(Box::new(garden_salad_1));
    my_kitchen.add_order(Box::new(garden_salad_2));
    my_kitchen.add_order(Box::new(caesar_salad));
    my_kitchen.add_order(Box::new(cherry_flambe));

    // 3. Process the orders
    my_kitchen.process_all();
}