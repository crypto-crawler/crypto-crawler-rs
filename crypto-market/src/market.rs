use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MarketType represents the type of a market
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MarketType {
    Spot,
    Futures,
    Swap,
    Option,
}

impl std::fmt::Display for MarketType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for MarketType {
    type Err = String;

    fn from_str(s: &str) -> ::std::result::Result<MarketType, Self::Err> {
        match s {
            "Spot" => ::std::result::Result::Ok(MarketType::Spot),
            "Futures" => ::std::result::Result::Ok(MarketType::Futures),
            "Swap" => ::std::result::Result::Ok(MarketType::Swap),
            "Option" => ::std::result::Result::Ok(MarketType::Option),
            _ => ::std::result::Result::Err(format!("{} is not a valid value for MarketType", s)),
        }
    }
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
    pub info: Value,
}
