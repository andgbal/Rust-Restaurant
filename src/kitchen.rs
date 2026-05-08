// src/kitchen.rs

use crate::menu::{HasBasicInfo, MenuItem, MenuItemLogic, OrderStatus};

pub struct Kitchen {
    pub orders: Vec<Box<dyn MenuItem>>,
}

impl Kitchen {
    // Constructor
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    // Add an order (Ownership moves from caller to Kitchen)
    pub fn add_order(&mut self, dish: Box<dyn MenuItem>) {
        self.orders.push(dish);
    }

    // The logic loop
    pub fn process_all(&mut self) {
        println!("--- KITCHEN STARTING SERVICE ---");
        // We iterate through the vector
        for dish in self.orders.iter_mut() {
            // dish is a Box<dyn MenuItem>, so we can call .serve()
            dish.serve(); 
        }
    }
}