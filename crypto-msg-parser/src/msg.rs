use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

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
    BBO,
    Ticker,
    Candlestick,
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
            /// the original message
            pub raw: Value,

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
    /// price
    pub price: f64,
    /// quantity
    pub quantity: f64,
    /// total traded value in USD(T)
    pub volume: f64,
    /// Which side is taker
    pub side: TradeSide,
    // Trade ID
    pub trade_id: String,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,
    /// the original message
    pub raw: Value,
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

#[derive(Serialize, Deserialize)]
pub struct OrderItem {
    price: f64,
    quantity: f64,
    cost: f64,
    timestamp: Option<i64>,
}

add_common_fields!(
    #[derive(Serialize, Deserialize)]
    struct OrderBookMsg {
        /// sorted from smallest to largest
        asks: Vec<OrderItem>,
        /// sorted from largest to smallest
        bids: Vec<OrderItem>,
        full: bool,
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

add_common_fields!(
    #[derive(Serialize, Deserialize)]
    struct FundingRateMsg {
        funding_rate: f64,
        funding_time: i64,
    }
);
