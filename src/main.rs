// src/main.rs
mod menu;
mod dishes;
mod kitchen;

use dishes::{Salad, Flambe};
use kitchen::Kitchen;

fn main() {
    let mut my_kitchen = Kitchen::new();

    // 1. Stock the Pantry!
    my_kitchen.pantry.add_item("Lettuce", 10);
    my_kitchen.pantry.add_item("Tomato", 10);
    my_kitchen.pantry.add_item("Cherry", 10);
    my_kitchen.pantry.add_item("Egg", 10);

    // 2. Create the Orders
    let garden_salad_1 = Salad::new("Garden Salad 1", 8.50);
    let garden_salad_2 = Salad::new("Garden Salad 2", 8.50); 
    let caesar_salad = Salad::new("Caesar Salad", 10.0); 
    let cherry_flambe = Flambe::new("Cherry Flambe", 14.0);
    
    my_kitchen.add_order(Box::new(garden_salad_1));
    my_kitchen.add_order(Box::new(garden_salad_2));
    my_kitchen.add_order(Box::new(caesar_salad));
    my_kitchen.add_order(Box::new(cherry_flambe));

    // 3. Run a 10-minute simulation
    my_kitchen.run_simulation(10);
}