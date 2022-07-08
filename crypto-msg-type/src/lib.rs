mod exchanges;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Crypto message types.
///
/// L2Snapshot and L2TopK are very similar, the former is from RESTful API,
/// the latter is from websocket.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
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
    /// Long/short ratio
    LongShortRatio,
    /// Taker buy/sell volume
    TakerVolume,
}

/// Translate to websocket subscribe/unsubscribe commands.
///
/// `configs` Some `msg_type` requires a config, for example,
/// `Candlestick` requires an `interval` parameter.
pub fn get_ws_commands(
    exchange: &str,
    msg_types: &[MessageType],
    symbols: &[String],
    subscribe: bool,
    configs: Option<&HashMap<String, String>>,
) -> Vec<String> {
    if msg_types.is_empty() || symbols.is_empty() {
        return Vec::new();
    }
    match exchange {
        "binance" => exchanges::binance::get_ws_commands(msg_types, symbols, subscribe, configs),
        "bitfinex" => exchanges::bitfinex::get_ws_commands(msg_types, symbols, subscribe, configs),
        "bitmex" => exchanges::bitmex::get_ws_commands(msg_types, symbols, subscribe, configs),
        "bybit" => exchanges::bybit::get_ws_commands(msg_types, symbols, subscribe, configs),
        "deribit" => exchanges::deribit::get_ws_commands(msg_types, symbols, subscribe, configs),
        "ftx" => exchanges::ftx::get_ws_commands(msg_types, symbols, subscribe, configs),
        "huobi" => exchanges::huobi::get_ws_commands(msg_types, symbols, subscribe, configs),
        "okex" => exchanges::okex::get_ws_commands(msg_types, symbols, subscribe, configs),
        "okx" => exchanges::okx::get_ws_commands(msg_types, symbols, subscribe, configs),
        _ => Vec::new(),
    }
}
