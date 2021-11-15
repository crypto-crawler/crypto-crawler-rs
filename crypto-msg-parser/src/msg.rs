use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::order::Order;

/// The type of a message
///
/// Copied from crypto_crawler::MessageType
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MessageType {
    Trade,
    L2Event,
    L2Snapshot,
    L3Event,
    L3Snapshot,
    #[serde(rename = "bbo")]
    #[allow(clippy::upper_case_acronyms)]
    BBO,
    Ticker,
    Candlestick,
    FundingRate,
}

macro_rules! add_common_fields {
    (
        $(#[$outer:meta])*
        struct $name:ident {
            $(
                $(#[$inner:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        $(#[$outer])*
        pub struct $name {
            /// The exchange name, unique for each exchage
            pub exchange: String,
            /// Market type
            pub market_type: MarketType,
            /// Exchange-specific trading symbol or id, recognized by RESTful API
            pub symbol: String,
            /// Unified pair, base/quote, e.g., BTC/USDT
            pub pair: String,
            /// Message type
            pub msg_type: MessageType,
            /// Unix timestamp, in milliseconds
            pub timestamp: i64,
            /// the original JSON message
            pub json: String,

            $(
                $(#[$inner])*
                pub $field: $ty
            ),*
        }
    };
}

add_common_fields!(
    /// Parent struct for all messages
    #[derive(Serialize, Deserialize)]
    struct Msg {}
);

/// Which side is taker
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TradeSide {
    /// Buyer is taker
    Buy,
    /// Seller is taker
    Sell,
}

/// Realtime trade message.
#[derive(Serialize, Deserialize)]
pub struct TradeMsg {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: String,
    /// Unified pair, base/quote, e.g., BTC/USDT
    pub pair: String,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,

    /// price
    pub price: f64,
    // Number of base coins
    pub quantity_base: f64,
    // Number of quote coins(mostly USDT)
    pub quantity_quote: f64,
    /// Number of contracts, always None for Spot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_contract: Option<f64>,
    /// Which side is taker
    pub side: TradeSide,
    // Trade ID
    pub trade_id: String,
    /// the original JSON message
    pub json: String,
}

/// Level2 orderbook message.
#[derive(Serialize, Deserialize)]
pub struct OrderBookMsg {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: String,
    /// Unified pair, base/quote, e.g., BTC/USDT
    pub pair: String,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,
    /// The first update id in the event
    pub seq_first: Option<u64>,
    /// The last update id in the event
    pub seq_last: Option<u64>,

    /// sorted in ascending order by price if snapshot=true, otherwise not sorted
    pub asks: Vec<Order>,
    /// sorted in descending order by price if snapshot=true, otherwise not sorted
    pub bids: Vec<Order>,
    // true means snapshot, false means updates
    pub snapshot: bool,

    /// the original JSON message
    pub json: String,
}

/// Funding rate message.
#[derive(Serialize, Deserialize)]
pub struct FundingRateMsg {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: String,
    /// Unified pair, base/quote, e.g., BTC/USDT
    pub pair: String,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,

    // Funding rate, which is calculated on data between [funding_time-16h, funding_time-8h]
    pub funding_rate: f64,
    // Funding time, the moment when funding rate is used
    pub funding_time: i64,
    // Estimated funding rate between [funding_time-h, funding_time], it will be static after funding_time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_rate: Option<f64>,
    /// the original JSON message
    pub json: String,
}

add_common_fields!(
    /// 24hr rolling window ticker
    #[derive(Serialize, Deserialize)]
    struct TickerMsg {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,

        quote_volume: f64,

        last_quantity: Option<f64>,

        best_bid_price: Option<f64>,
        best_bid_quantity: Option<f64>,
        best_ask_price: Option<f64>,
        best_ask_quantity: Option<f64>,

        /// availale in Futures and Swap markets
        open_interest: Option<f64>,
        /// availale in Futures and Swap markets
        open_interest_quote: Option<f64>,
    }
);

add_common_fields!(
    #[derive(Serialize, Deserialize)]
    struct BboMsg {
        bid_price: f64,
        bid_quantity: f64,
        ask_price: f64,
        ask_quantity: f64,
    }
);

add_common_fields!(
    #[derive(Serialize, Deserialize)]
    struct KlineMsg {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        /// base volume
        volume: f64,
        /// m, minute; H, hour; D, day; W, week; M, month; Y, year
        period: String,
        /// quote volume
        quote_volume: Option<f64>,
    }
);
