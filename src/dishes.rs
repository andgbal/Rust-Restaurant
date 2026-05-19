use crate::menu::{HasBasicInfo, MenuItemLogic, OrderStatus};

// --- DISH 1: THE SALAD (The "Lazy" Subclass) ---
pub struct Salad {
    name: String,
    price: f32,
    stock: i32,
    std_serve_time: u64,
    status: OrderStatus
}

impl Salad {
    pub fn new(name: &str, price: f32, stock: i32) -> Self { // Made pub
        Self {
            name: name.to_string(),
            price,
            stock,
            status: OrderStatus::Pending,
            std_serve_time: 4
        }
    }
}

impl HasBasicInfo for Salad {
    fn name(&self) -> &str { &self.name }
    fn price(&self) -> f32 { self.price }
    fn stock_count(&self) -> i32 { self.stock}
    fn modify_stock(&mut self, amount: i32) { self.stock += amount }
    fn modify_status (&mut self, value: OrderStatus) {
        self.status = value;
        self.print_status();
    }
    fn get_status (&self) -> &OrderStatus { &self.status }
    fn get_std_serve_time(&self) -> u64 {self.std_serve_time}
}
impl MenuItemLogic for Salad {} // Uses default serve_logic

// --- DISH 2: THE FLAMBÉ (The "Overriding" Subclass) ---
pub struct Flambe {
    name: String,
    price: f32,
    stock: i32,
    std_serve_time: u64,
    status: OrderStatus
}

impl Flambe {
    pub fn new(name: &str, price: f32, stock: i32) -> Self { // Made pub
        Self {
            name: name.to_string(),
            price,
            stock,
            status: OrderStatus::Pending,
            std_serve_time: 8
        }
    }
}

impl HasBasicInfo for Flambe {
    fn name(&self) -> &str { &self.name }
    fn price(&self) -> f32 { self.price }
    fn stock_count(&self) -> i32 { self.stock}
    fn modify_stock(&mut self, amount: i32) { self.stock += amount }
    fn modify_status (&mut self, value: OrderStatus) {
        self.status = value;
        self.print_status();
    }
    fn get_std_serve_time(&self) -> u64 {self.std_serve_time}
    fn get_status (&self) -> &OrderStatus { &self.status }
}

impl MenuItemLogic for Flambe {
    fn serve_logic(&self) {
        println!("🔥 FIRE! Lighting the {} at the table! 🔥", self.name());
    }

    fn discount_percentage(&self) -> f32{
        0.8
    }
}