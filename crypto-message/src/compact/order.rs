use serde::{Deserialize, Serialize};

/// Choose which field to use as the quantity.
#[derive(Copy, Clone)]
pub enum QuantityChoice {
    /// quantity_base
    Base,
    /// quantity_quote
    Quote,
    /// quantity_contract
    Contract,
}

/// An order in the orderbook asks or bids array.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Order {
    /// price
    pub price: f64,
    // quantity, comes from one of quantity_base, quantity_quote and quantity_contract.
    pub quantity: f64,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.quantity == other.quantity
    }
}

impl Eq for Order {}
