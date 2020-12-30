use crypto_markets::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

/// ChannelType represents the type of a channel
#[derive(PartialEq, Serialize, Deserialize, Display, EnumString)]
pub enum ChannelType {
    BBO,
    Kline,
    OrderBook,
    Ticker,
    Trade,
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
            /// Unified pair, from Market.pair, e.g., BTC_USDT
            pub pair: String,
            /// Exchange specific pair, from Market.id
            pub raw_pair: String,
            // Original websocket channel
            pub channel: String,
            /// Channel type
            pub channel_type: ChannelType,
            /// Unix timestamp, in milliseconds
            pub timestamp: i64,
            /// the original message
            pub raw: HashMap<String, Value>,

            $(
                $(#[$inner])*
                pub $field: $ty
            ),*
        }
    };
}

add_common_fields!(
    /// Per-trade message
    #[derive(Serialize, Deserialize)]
    struct TradeMsg {
        price: f64,
        quantity: f64,
        /// true, ask; false, bid
        side: bool,
        trade_id: String,
    }
);

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
    pub price: f64,
    pub quantity: f64,
    pub cost: f64,
    pub timestamp: Option<i64>,
}

add_common_fields!(
    /// Orderbook
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
    /// Best Bid and Offer
    #[derive(Serialize, Deserialize)]
    struct BboMsg {
        bid_price: f64,
        bid_quantity: f64,
        ask_price: f64,
        ask_quantity: f64,
    }
);

add_common_fields!(
    // Kline, a.k.a, candlestick
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
    /// Funding rate of Swap contract
    #[derive(Serialize, Deserialize)]
    struct FundingRateMsg {
        funding_rate: f64,
        funding_time: i64,
    }
);

/// Msg is a container to store multiple kinds of messages.
pub enum Msg {
    BBO(BboMsg),
    Kline(KlineMsg),
    OrderBook(OrderBookMsg),
    Ticker(TickerMsg),
    Trade(TradeMsg),
    FundingRate(FundingRateMsg),
}
