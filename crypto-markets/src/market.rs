use crate::MarketType;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
    /// exchange-specific trading symbol, used by RESTful API
    pub symbol: String,
    /// unified pair of trading currencies, e.g., BTC_USDT
    pub pair: String,
    /// unified base currency, e.g., BTC
    pub base: String,
    /// unified quote currency, e.g., USDT
    pub quote: String,
    // settled currency
    pub settle: String,
    /// exchange-specific base currency
    pub base_id: String,
    /// exchange-specific quote currency
    pub quote_id: String,
    /// market status
    pub active: bool,
    /// Margin enabled.
    ///
    /// * All contract markets are margin enabled, including future, swap and option.
    /// * Only a few exchanges have spot market with margin enabled.
    pub margin: bool,
    pub fees: Fees,
    /// number of decimal digits "after the dot"
    pub precision: Precision,
    /// minimum quantity when placing orders
    pub min_quantity: MinQuantity,
    // The value of one contract, not applicable to sport markets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_value: Option<f64>,
    /// Delivery date, unix timestamp in milliseconds, only applicable for future and option markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<u64>,
    /// the original JSON string retrieved from the exchange
    pub info: Map<String, Value>,
}
