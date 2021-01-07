use super::MarketType;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros::{Display, EnumString};

/// DataType represents the type of a message
#[derive(PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
pub enum MessageType {
    Trade,
    L2Event,
    L2Snapshot,
    L3Event,
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

impl Message {
    pub fn new(
        exchange: String,
        market_type: MarketType,
        symbol: String,
        msg_type: MessageType,
        json: String,
    ) -> Self {
        Message {
            exchange,
            market_type,
            symbol,
            msg_type,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            json,
        }
    }
}
