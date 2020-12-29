use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

/// MarketType represents the type of a market
#[derive(Serialize, Deserialize, Display, EnumString)]
pub enum MarketType {
    Spot,
    Futures,
    Swap,
    Option,
}

#[derive(Serialize, Deserialize)]
pub struct Fees {
    pub maker: f64,
    pub taker: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Precision {
    pub price: i64,
    pub base: i64,
    pub quote: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct MinQuantity {
    pub base: Option<f64>,
    pub quote: Option<f64>,
}

/// Market contains all information about a market
#[derive(Serialize, Deserialize)]
pub struct Market {
    /// exchange name
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// exchange-specific pair of trading currencies if Spot, or raw symbol if Fetures, Swap, Option
    pub id: String,
    /// unified pair of trading currencies, e.g., BTC_USDT
    pub pair: String,
    /// unified base currency, e.g., BTC
    pub base: String,
    /// unified quote currency, e.g., USDT
    pub quote: String,
    /// exchange-specific base currency
    pub base_id: String,
    /// exchange-specific quote currency
    pub quote_id: String,
    /// market status
    pub active: bool,
    pub fees: Fees,
    /// number of decimal digits "after the dot"
    pub precision: Precision,
    /// minimum quantity when placing orders
    pub min_quantity: MinQuantity,
    /// the original JSON string retrieved from the exchange
    pub raw: HashMap<String, Value>,
}
