// src/kitchen.rs
use crate::menu::{MenuItem, OrderStatus};
use std::{thread, time::Duration, collections::HashMap};

// --- PANTRY SYSTEM ---
pub struct Inventory {
    stock: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { stock: HashMap::new() }
    }

    pub fn add_item(&mut self, name: &str, quantity: u32) {
        let counter = self.stock.entry(name.to_string()).or_insert(0);
        *counter += quantity;
    }

    pub fn try_consume_ingredients(&mut self, recipe: Vec<(String, u32)>) -> bool {
        // Pass 1: Check availability
        for (ingredient, amount_needed) in &recipe {
            let current_stock = self.stock.get(ingredient).unwrap_or(&0);
            if *current_stock < *amount_needed {
                return false; 
            }
        }
        // Pass 2: Deduct
        for (ingredient, amount_needed) in recipe {
            if let Some(current_stock) = self.stock.get_mut(&ingredient) {
                *current_stock -= amount_needed;
            }
        }
        true
    }
}

// --- KITCHEN SYSTEM ---
pub struct Kitchen {
    pub orders: Vec<Box<dyn MenuItem>>,
    pub pantry: Inventory,
    total_revenue: f64,
    current_time: u32,
}

impl Kitchen {
    pub fn new() -> Self {
        Self { 
            orders: Vec::new(),
            pantry: Inventory::new(),
            total_revenue: 0.0,
            current_time: 0,
        }
    }

    pub fn add_order(&mut self, dish: Box<dyn MenuItem>) {
        self.orders.push(dish);
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
        thread::sleep(Duration::from_millis(500)); 
        println!("\n--- [Minute {}] ---", self.current_time);

        // 1. Process all dishes in the queue
        for dish in self.orders.iter_mut() {
            
            // We use a hacky clone of the status here to avoid borrowing issues during the match
            let current_status = match dish.get_status() {
                OrderStatus::Pending => OrderStatus::Pending,
                OrderStatus::Preparing { minutes_remaining } => OrderStatus::Preparing { minutes_remaining: *minutes_remaining },
                _ => continue, // Ignore already served/cancelled items
            };

            match current_status {
                OrderStatus::Pending => {
                    let recipe = dish.required_ingredients();
                    if self.pantry.try_consume_ingredients(recipe) {
                        println!("[CHEF] Starting: {}", dish.name());
                        // Move to preparing using the dish's built-in time!
                        dish.modify_status(OrderStatus::Preparing { minutes_remaining: dish.get_std_serve_time() });
                    } else {
                        println!("[CHEF] Cannot make {}: Out of stock!", dish.name());
                        dish.modify_status(OrderStatus::Cancelled("Out of stock".to_string()));
                    }
                }
                
                OrderStatus::Preparing { minutes_remaining } => {
                    if minutes_remaining <= 1 {
                        // Food is done! Serve it and take the money.
                        dish.serve_logic();
                        dish.modify_status(OrderStatus::Served);
                        
                        let final_price = dish.price() * (1.0 - dish.discount_percentage());
                        self.total_revenue += final_price as f64;
                    } else {
                        // Keep cooking (decrease time by 1)
                        dish.modify_status(OrderStatus::Preparing { minutes_remaining: minutes_remaining - 1 });
                    }
                }
                _ => {}
            }
        }

        // 2. Clean up the queue (Remove Served and Cancelled items to save memory)
        self.orders.retain(|dish| {
            matches!(dish.get_status(), OrderStatus::Pending | OrderStatus::Preparing { .. })
        });
    }

    pub fn run_simulation(&mut self, shift_length_minutes: u32) {
        println!("=== RESTAURANT OPENING FOR {} MINUTE SHIFT ===", shift_length_minutes);
        while self.current_time < shift_length_minutes {
            self.tick();
        }
        println!("\n=== RESTAURANT CLOSED ===");
        println!("[SYSTEM] Active orders remaining: {}", self.orders.len());
        println!("[FINANCE] Total Revenue Generated: ${:.2}", self.total_revenue);
    }
}