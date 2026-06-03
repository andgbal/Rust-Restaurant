// src/menu.rs

use std::thread::sleep;
use std::time::Duration;

pub enum OrderStatus {
    Pending,
    Preparing { minutes_remaining: u64 },
    Served,
    Cancelled(String), // Carries a reason for cancellation
}

// The Data Requirement (Getters)
pub trait HasBasicInfo {
    fn name(&self) -> &str;
    fn price(&self) -> f32;
    fn get_std_serve_time(&self) -> u64;
    fn get_status(&self) -> &OrderStatus;
    fn modify_status(&mut self, value: OrderStatus);
    fn required_ingredients(&self) -> Vec<(String, u32)>;
}

// The Overridable Logic (Virtual-like)
pub trait MenuItemLogic: HasBasicInfo {
    fn serve_logic(&self) {
        println!("Standard Service: Placing {} on the table.", self.name());    
    }

    fn discount_percentage(&self) -> f32 {
        0.0
    }

    fn print_status(&self) {
        // We match on the value of self.status
        match &self.get_status() {
            OrderStatus::Pending => {
                println!("Status: Waiting for chef...");
            }
            OrderStatus::Preparing { minutes_remaining } => {
                // Rust 'pulls' the minutes out of the enum here
                println!("Status: Coming in {} minutes", minutes_remaining);
            }
            OrderStatus::Served => {
                println!("Status: Served already");
            }
            OrderStatus::Cancelled(reason) => {
                // Rust 'pulls' the String reason out here
                println!("Status: Sorry, we can't make this because: {}", reason);
            }
        }
    }
}

// The Unchangeable Manager (Blanket)
// pub trait MenuItem: HasBasicInfo + MenuItemLogic {
//     fn serve(&mut self) -> Result<(), String>{
//         self.modify_status(OrderStatus::Pending);
//         println!("--- Restaurant Order ---");
//         if self.stock_count() <= 0 {
//             println!("Request order {} out of stock!", self.name());
//             self.modify_status(OrderStatus::Cancelled("Out of stock".to_string()));
//             self.modify_stock(1);
//             return Err("Out of stock".to_string());
//         }
//         else{
            
//             self.serve_logic();
//             self.modify_stock(-1);

//             self.modify_status(OrderStatus::Preparing { minutes_remaining: 8});
//             println!("[COOKING] Starting work on: {}", self.name());
//             sleep(Duration::from_secs(self.get_std_serve_time()));
            
//             if self.discount_percentage() != 0.0{
//                 println!("Congrats! Got a discount price for only ${:.2}", self.discount_percentage() * self.price())
//             }
//             else{
//                 println!("Price for ${:.2}", self.price())
//             }
//             self.modify_status(OrderStatus::Served);
//             Ok(())
//         }
//     }
// }

// Automate the MenuItem trait for anyone meeting the requirements
pub trait MenuItem: HasBasicInfo + MenuItemLogic {}
impl<T: HasBasicInfo + MenuItemLogic> MenuItem for T {}