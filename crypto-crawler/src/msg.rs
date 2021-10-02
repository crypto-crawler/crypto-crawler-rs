use super::MarketType;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};
use strum_macros::{Display, EnumString};

/// Message types.
///
/// L2Snapshot and L2TopK are very similar, the former is from RESTful API, the latter is from websocket.
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MessageType {
    /// tick-by-tick trade messages
    Trade,
    /// Incremental level2 orderbook updates
    L2Event,
    /// Level2 snapshot from RESTful API
    L2Snapshot,
    /// Level2 top K snapshots from websocket
    #[serde(rename = "l2_topk")]
    #[strum(serialize = "l2_topk")]
    L2TopK,
    /// Incremental level3 orderbook updates
    L3Event,
    /// Level3 snapshot from RESTful API
    L3Snapshot,
    /// Best bid and ask
    #[serde(rename = "bbo")]
    #[allow(clippy::upper_case_acronyms)]
    BBO,
    /// 24hr rolling window ticker
    Ticker,
    /// OHLCV candlestick
    Candlestick,
    /// Funding rate
    FundingRate,
}

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
