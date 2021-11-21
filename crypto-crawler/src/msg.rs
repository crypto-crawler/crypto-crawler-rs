use super::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

/// Message represents messages received by crawlers.
#[derive(Serialize, Deserialize)]
pub struct Message {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp in milliseconds
    pub received_at: u64,
    /// the original message
    pub json: String,
}

impl Message {
    pub fn new(
        exchange: String,
        market_type: MarketType,
        msg_type: MessageType,
        json: String,
    ) -> Self {
        Message {
            exchange,
            market_type,
            msg_type,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
            json,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
