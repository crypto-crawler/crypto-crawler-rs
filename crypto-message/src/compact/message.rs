pub use crate::compact::{Order, QuantityChoice};
use crate::TradeSide;
use ahash::{CallHasher, RandomState};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use strum_macros::{Display, EnumString};

use super::order::Float;

/// Cryptocurrency exchanges.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Exchange {
    Binance,
    Bitfinex,
    Bitget,
    Bithumb,
    Bitmex,
    Bitstamp,
    Bybit,
    CoinbasePro,
    Deribit,
    Dydx,
    Ftx,
    Gate,
    Huobi,
    Kraken,
    Kucoin,
    Mexc,
    Okx,
}

/// Message represents multiple types of messages.
#[derive(Eq, PartialEq, Debug)]
pub enum Message {
    Trade(TradeMsg),
    Bbo(BboMsg),
    Level2(OrderBookMsg), // Level2, L2TopK, L2Snapshot
    FundingRate(FundingRateMsg),
    Candlestick(CandlestickMsg),
    Ticker(TickerMsg),
}

impl Message {
    pub fn get_timestamp(&self) -> i64 {
        match self {
            Message::Trade(trade) => trade.timestamp,
            Message::Bbo(bbo) => bbo.timestamp,
            Message::Level2(level2) => level2.timestamp,
            Message::FundingRate(funding_rate) => funding_rate.timestamp,
            Message::Candlestick(candlestick) => candlestick.timestamp,
            Message::Ticker(ticker) => ticker.timestamp,
        }
    }
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
            pub exchange: Exchange,
            /// Market type
            pub market_type: MarketType,
            /// Exchange-specific trading symbol or id, recognized by RESTful API
            pub symbol: u64,
            /// Message type
            pub msg_type: MessageType,
            /// Unix timestamp, in milliseconds
            pub timestamp: i64,

            $(
                $(#[$inner])*
                pub $field: $ty
            ),*
        }
    };
}

/// Realtime trade message.
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeMsg {
    /// The exchange name, unique for each exchage
    pub exchange: Exchange,
    /// Market type
    pub market_type: MarketType,
    /// Message type
    pub msg_type: MessageType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: u64,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,

    /// Which side is taker
    pub side: TradeSide,
    /// price
    pub price: f64,
    /// quantity, comes from one of quantity_base, quantity_quote and quantity_contract.
    pub quantity: f64,
}

/// Level2 orderbook message.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookMsg {
    /// The exchange name, unique for each exchage
    pub exchange: Exchange,
    /// Market type
    pub market_type: MarketType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: u64,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,
    // true means snapshot, false means updates
    pub snapshot: bool,
    /// sorted in ascending order by price if snapshot=true, otherwise not sorted
    pub asks: Vec<Order>,
    /// sorted in descending order by price if snapshot=true, otherwise not sorted
    pub bids: Vec<Order>,
}

/// Funding rate message.
#[derive(Serialize, Deserialize, Debug)]
pub struct FundingRateMsg {
    /// The exchange name, unique for each exchage
    pub exchange: Exchange,
    /// Market type
    pub market_type: MarketType,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: u64,
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
}

add_common_fields!(
    /// 24hr rolling window ticker
    #[derive(Serialize, Deserialize, Debug)]
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
    #[derive(Serialize, Deserialize, Debug)]
    struct BboMsg {
        bid_price: f64,
        bid_quantity: f64,

        ask_price: f64,
        ask_quantity: f64,
    }
);

add_common_fields!(
    #[derive(Serialize, Deserialize, Debug)]
    struct CandlestickMsg {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        /// m, minute; H, hour; D, day; W, week; M, month; Y, year
        period: String,
    }
);

// Convert from parsed message in lib.rs.

/// Convers a string to u64 hash.
///
/// Exported for unit test purpose.
pub fn calculate_hash(s: &str) -> u64 {
    let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
    str::get_hash(s, &build_hasher)
}

impl TradeMsg {
    pub fn from_json(msg: &crate::TradeMsg, quantity_choice: QuantityChoice) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();

        TradeMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            price: msg.price,
            quantity: match quantity_choice {
                QuantityChoice::Base => msg.quantity_base,
                QuantityChoice::Quote => msg.quantity_quote,
                QuantityChoice::Contract => msg.quantity_contract.unwrap(),
            },
            side: msg.side,
        }
    }
}

fn convert_order(order: &crate::Order, quantity_choice: QuantityChoice) -> Order {
    Order {
        price: order.price as Float,
        quantity: (match quantity_choice {
            QuantityChoice::Base => order.quantity_base,
            QuantityChoice::Quote => order.quantity_quote,
            QuantityChoice::Contract => order.quantity_contract.unwrap(),
        }) as Float,
    }
}

impl OrderBookMsg {
    pub fn from_json(msg: &crate::OrderBookMsg, quantity_choice: QuantityChoice) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();

        OrderBookMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            snapshot: msg.snapshot,
            asks: msg
                .asks
                .iter()
                .map(|order| convert_order(order, quantity_choice))
                .collect(),
            bids: msg
                .bids
                .iter()
                .map(|order| convert_order(order, quantity_choice))
                .collect(),
        }
    }
}

impl BboMsg {
    pub fn from_json(msg: &crate::BboMsg, quantity_choice: QuantityChoice) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();

        BboMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            bid_price: msg.bid_price,
            bid_quantity: match quantity_choice {
                QuantityChoice::Base => msg.bid_quantity_base,
                QuantityChoice::Quote => msg.bid_quantity_quote,
                QuantityChoice::Contract => msg.bid_quantity_contract.unwrap(),
            },
            ask_price: msg.ask_price,
            ask_quantity: match quantity_choice {
                QuantityChoice::Base => msg.ask_quantity_base,
                QuantityChoice::Quote => msg.ask_quantity_quote,
                QuantityChoice::Contract => msg.ask_quantity_contract.unwrap(),
            },
        }
    }
}

impl TickerMsg {
    pub fn from_json(msg: &crate::TickerMsg) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();
        TickerMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            open: msg.open,
            high: msg.high,
            low: msg.low,
            close: msg.close,
            volume: msg.volume,
            quote_volume: msg.quote_volume,
            last_quantity: msg.last_quantity,
            best_bid_price: msg.best_bid_price,
            best_bid_quantity: msg.best_bid_quantity,
            best_ask_price: msg.best_ask_price,
            best_ask_quantity: msg.best_ask_quantity,
            open_interest: msg.open_interest,
            open_interest_quote: msg.open_interest_quote,
        }
    }
}

impl CandlestickMsg {
    pub fn from_json(msg: &crate::CandlestickMsg) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();
        CandlestickMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            open: msg.open,
            high: msg.high,
            low: msg.low,
            close: msg.close,
            volume: msg.volume,
            period: msg.period.clone(),
        }
    }
}

impl FundingRateMsg {
    pub fn from_json(msg: &crate::FundingRateMsg) -> Self {
        let exchange = Exchange::from_str(&msg.exchange).unwrap();

        FundingRateMsg {
            exchange,
            market_type: msg.market_type,
            msg_type: msg.msg_type,
            symbol: calculate_hash(&msg.symbol),
            timestamp: msg.timestamp,
            funding_rate: msg.funding_rate,
            funding_time: msg.funding_time,
            estimated_rate: msg.estimated_rate,
        }
    }
}

// ##### impl Ord #####

macro_rules! impl_partial_ord {
    ($struct_name:ident) => {
        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.timestamp.cmp(&other.timestamp))
            }
        }
    };
}

macro_rules! impl_ord {
    ($struct_name:ident) => {
        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.timestamp.cmp(&other.timestamp)
            }
        }
    };
}

// TradeMsg
impl PartialEq for TradeMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.price == other.price
            && self.quantity == other.quantity
    }
}

impl Eq for TradeMsg {}

impl_partial_ord!(TradeMsg);
impl_ord!(TradeMsg);

// OrderBookMsg
impl PartialEq for OrderBookMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.asks == other.asks
            && self.bids == other.bids
    }
}

impl Eq for OrderBookMsg {}

impl_partial_ord!(OrderBookMsg);
impl_ord!(OrderBookMsg);

// BboMsg
impl PartialEq for BboMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.bid_price == other.bid_price
            && self.bid_quantity == other.bid_quantity
            && self.ask_price == other.ask_price
            && self.ask_quantity == other.ask_quantity
    }
}

impl Eq for BboMsg {}

impl_partial_ord!(BboMsg);
impl_ord!(BboMsg);

// TickerMsg
impl PartialEq for TickerMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.open == other.open
            && self.high == other.high
            && self.close == other.close
            && self.volume == other.volume
            && self.quote_volume == other.quote_volume
    }
}

impl Eq for TickerMsg {}

impl_partial_ord!(TickerMsg);
impl_ord!(TickerMsg);

// CandlestickMsg
impl PartialEq for CandlestickMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.open == other.open
            && self.high == other.high
            && self.close == other.close
            && self.volume == other.volume
            && self.period == other.period
    }
}

impl Eq for CandlestickMsg {}

impl_partial_ord!(CandlestickMsg);
impl_ord!(CandlestickMsg);

// FundingRateMsg
impl PartialEq for FundingRateMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.funding_rate == other.funding_rate
            && self.funding_time == other.funding_time
    }
}

impl Eq for FundingRateMsg {}

impl_partial_ord!(FundingRateMsg);

impl Ord for FundingRateMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ret = self.timestamp.cmp(&other.timestamp);
        if ret == std::cmp::Ordering::Equal {
            self.funding_time.cmp(&other.funding_time)
        } else {
            ret
        }
    }
}

// Message
impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let this_timestamp = self.get_timestamp();
        let other_timestamp = other.get_timestamp();
        Some(this_timestamp.cmp(&other_timestamp))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_timestamp = self.get_timestamp();
        let other_timestamp = other.get_timestamp();
        this_timestamp.cmp(&other_timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_hash;

    #[test]
    fn test_calculate_hash() {
        let hash = calculate_hash("BTCUSDT");
        assert_eq!(12658250145686044913, hash);
    }
}
