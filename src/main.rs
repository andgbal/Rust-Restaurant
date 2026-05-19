// src/main.rs

mod menu;
mod dishes;
mod kitchen;


use dishes::{Salad, Flambe};
use kitchen::{Kitchen};

fn main() {
    let mut my_kitchen = Kitchen::new();

    // 1. Create the dish
    let salad = Salad::new("Garden Salad", 8.50, 0); 
    let Flambe = Flambe::new("Garden Salad", 14.0, 2); 
    
    // 2. Wrap it in a Box and add it to the kitchen
    // This MOVEs the salad into the kitchen's vector
    my_kitchen.add_order(Box::new(salad));
    my_kitchen.add_order(Box::new(Flambe));

    // 3. Process the orders
    my_kitchen.process_all();
}