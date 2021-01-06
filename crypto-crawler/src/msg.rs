use super::MarketType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// DataType represents the type of a message
#[derive(PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
pub enum MessageType {
    Trade,
    L2Update,
    L2Snapshot,
    L3Update,
    L3Snapshot,
    BBO,
    Ticker,
    Candlestick,
}

/// Message represents messages received by crawlers.
#[derive(Serialize, Deserialize)]
pub struct Message {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Exchange specific symbol
    pub symbol: String,
    /// Data type
    pub msg_type: MessageType,
    /// Unix timestamp in milliseconds
    pub received_at: u128,
    /// the original message
    pub json: String,
}
