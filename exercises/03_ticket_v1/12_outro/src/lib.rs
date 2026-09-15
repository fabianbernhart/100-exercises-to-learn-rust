// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

use std::ops::Mul;

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: i64,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: i64) -> Self {
        if product_name.len() > 300 {
            panic!("product_name cant be logger than 300 bytes");
        }
        if quantity < 1 {
            panic!("quantity must be striclty greater than zero");
        }

        if unit_price < 1 {
            panic!("unit_price must be striclty greater than zero");
        }

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn unit_price(&self) -> &i64 {
        &self.unit_price
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn total(&self) -> i64 {
        self.quantity as i64 * self.unit_price as i64
    }

    pub fn set_unit_price(&mut self, unit_price: i64) {
        self.unit_price = unit_price;
    }

    pub fn set_product_name(&mut self, product_name: String) {
        self.product_name = product_name
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity
    }
}
