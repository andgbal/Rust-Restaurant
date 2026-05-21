// src/kitchen.rs

use crate::menu::{MenuItem, OrderStatus};

pub struct Kitchen {
    pub orders: Vec<Box<dyn MenuItem>>,
    total_revenue: f64,
}

impl Kitchen {
    // Constructor
    pub fn new() -> Self {
        Self { 
            orders: Vec::new(),
            total_revenue: 0.0,
        }
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
            match dish.serve(){
                Err(reason) => {
                    println!("[!] KITCHEN ALERT: {}", reason);
                }

                Ok(()) => {
                    let discount = dish.discount_percentage();

                    let final_price = if discount == 0.0 {
                        dish.price() // No discount, charge full price
                    } else {
                        dish.price() * discount // Or however your discount math is structured
                    };

                    self.total_revenue += final_price as f64;
                }
            }
        }

        self.orders.retain(|dish| {
            let status = dish.get_status();
            
            // We match to avoid comparing internal data of variants
            match status {
                OrderStatus::Served => false,       // Drop it!
                OrderStatus::Cancelled(_) => false,  // Drop it!
                _ => true,                          // Keep Pending or Preparing
            }
        });

        // 3. Print the remaining queue size to verify it worked
        println!("[SYSTEM] Active orders remaining in kitchen queue: {}", self.orders.len());
        println!("[FINANCE] Total Revenue Generated this Shift: ${:.2}", self.total_revenue);
    }
}

// src/kitchen.rs (at the very bottom)

// 1. This tells the compiler: "Only compile this if we are running tests!"
#[cfg(test)]
mod tests {
    // 2. Import everything from the parent file (kitchen.rs)
    use super::*; 
    // 3. Import the dishes we need to test
    use crate::dishes::{Salad, Flambe}; 

    // 4. This attribute flags the function as a test case
    #[test] 
    fn test_kitchen_revenue_and_queue_cleanup() {
        // Setup: Create a kitchen
        let mut test_kitchen = Kitchen::new();
        
        // Setup: Add one $10 Salad and one $20 Flambé (which has a 20% discount)
        test_kitchen.add_order(Box::new(Salad::new("Test Salad", 10.0, 5)));
        test_kitchen.add_order(Box::new(Flambe::new("Test Flambe", 20.0, 5)));

        // Action: Run the service
        test_kitchen.process_all();

        // Assert 1: The queue should be perfectly empty (0)
        assert_eq!(test_kitchen.orders.len(), 0, "The queue did not clear!");

        // Assert 2: Total revenue should be exactly $26.00 ($10 + $16)
        assert_eq!(test_kitchen.total_revenue, 26.0, "Revenue calculation is wrong!");
    }
}