use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Crypto message types.
///
/// L2Snapshot and L2TopK are very similar, the former is from RESTful API,
/// the latter is from websocket.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MessageType {
    /// All other messages
    Other,
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
    /// Open interest
    OpenInterest,
}
