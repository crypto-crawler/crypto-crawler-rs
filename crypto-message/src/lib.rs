mod order;

pub use crate::order::Order;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[deprecated(
    note = "Use `CandlestickMsg` instead. This is an alias for `CandlestickMsg`."
)]
pub use self::CandlestickMsg as KlineMsg;

/// Message represents multiple types of messages.
#[derive(Eq, PartialEq)]
pub enum Message {
    Trade(TradeMsg),
    Bbo(BboMsg),
    L2Event(OrderBookMsg),
    FundingRate(FundingRateMsg),
    Candlestick(CandlestickMsg),
    Ticker(TickerMsg),
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

/// Which side is taker
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Display, Debug, EnumString)]
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
    /// Message type
    pub msg_type: MessageType,
    /// Unified pair, base/quote, e.g., BTC/USDT
    pub pair: String,
    /// Exchange-specific trading symbol or id, recognized by RESTful API
    pub symbol: String,
    /// Unix timestamp, in milliseconds
    pub timestamp: i64,

    /// Which side is taker
    pub side: TradeSide,
    /// price
    pub price: f64,
    // Number of base coins
    pub quantity_base: f64,
    // Number of quote coins(mostly USDT)
    pub quantity_quote: f64,
    /// Number of contracts, always None for Spot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_contract: Option<f64>,
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
    // true means snapshot, false means updates
    pub snapshot: bool,
    /// sorted in ascending order by price if snapshot=true, otherwise not sorted
    pub asks: Vec<Order>,
    /// sorted in descending order by price if snapshot=true, otherwise not sorted
    pub bids: Vec<Order>,
    /// The sequence ID for this update (not all exchanges provide this information)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_id: Option<u64>,
    /// The sequence ID for the previous update (not all exchanges provide this information)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_seq_id: Option<u64>,
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
        bid_quantity_base: f64,
        bid_quantity_quote: f64,
        bid_quantity_contract: Option<f64>,

        ask_price: f64,
        ask_quantity_base: f64,
        ask_quantity_quote: f64,
        ask_quantity_contract: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<u64>,
    }
);

add_common_fields!(
    #[derive(Serialize, Deserialize)]
    struct CandlestickMsg {
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

// CSV utilities.

const PRECISION: f64 = 1000000000.0; // 9 decimals

fn round(f: f64) -> f64 {
    (f * PRECISION).round() / PRECISION
}

impl TradeMsg {
    /// Convert to a CSV string.
    ///
    /// The `exchange`, `market_type`, `msg_type`, `pair` and `symbol` fields are not
    /// included to save some disk space.
    pub fn to_csv_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.timestamp,
            self.side,
            self.price,
            round(self.quantity_base),
            round(self.quantity_quote),
            if let Some(x) = self.quantity_contract {
                round(x).to_string()
            } else {
                "".to_string()
            },
            self.trade_id,
            self.json
        )
    }

    /// Convert from a CSV string.
    pub fn from_csv_string(
        exchange: &str,
        market_type: &str,
        msg_type: &str,
        pair: &str,
        symbol: &str,
        s: &str,
    ) -> Self {
        let v: Vec<&str> = s.split('\t').collect();
        assert_eq!(8, v.len());
        let market_type = MarketType::from_str(market_type).unwrap();
        let msg_type = MessageType::from_str(msg_type).unwrap();
        let side = TradeSide::from_str(v[1]).unwrap();
        let price = v[2].parse::<f64>().unwrap();
        let quantity_base = v[3].parse::<f64>().unwrap();
        let quantity_quote = v[4].parse::<f64>().unwrap();
        let quantity_contract = if v[5].is_empty() {
            None
        } else {
            Some(v[5].parse::<f64>().unwrap())
        };

        TradeMsg {
            exchange: exchange.to_string(),
            market_type,
            msg_type,
            pair: pair.to_string(),
            symbol: symbol.to_string(),
            timestamp: v[0].parse::<i64>().unwrap(),
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
            side,
            trade_id: v[6].to_string(),
            json: v[7].to_string(),
        }
    }
}

impl OrderBookMsg {
    /// Convert to a CSV string.
    ///
    /// The `exchange`, `market_type`, `msg_type`, `pair` and `symbol` fields are not
    /// included to save some disk space.
    pub fn to_csv_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            self.timestamp,
            self.snapshot,
            serde_json::to_string(&self.asks).unwrap(),
            serde_json::to_string(&self.bids).unwrap(),
            self.seq_id.map(|x| x.to_string()).unwrap_or_default(),
            self.prev_seq_id.map(|x| x.to_string()).unwrap_or_default()
        )
    }

    /// Convert from a CSV string.
    pub fn from_csv_string(
        exchange: &str,
        market_type: &str,
        msg_type: &str,
        pair: &str,
        symbol: &str,
        s: &str,
    ) -> Self {
        let v: Vec<&str> = s.split('\t').collect();
        assert_eq!(6, v.len());
        let market_type = MarketType::from_str(market_type).unwrap();
        let msg_type = MessageType::from_str(msg_type).unwrap();
        let asks = serde_json::from_str::<Vec<Order>>(v[2]).unwrap();
        let bids = serde_json::from_str::<Vec<Order>>(v[3]).unwrap();
        let seq_id = if v[4].is_empty() {
            None
        } else {
            Some(v[4].parse::<u64>().unwrap())
        };
        let prev_seq_id = if v[5].is_empty() {
            None
        } else {
            Some(v[5].parse::<u64>().unwrap())
        };

        OrderBookMsg {
            exchange: exchange.to_string(),
            market_type,
            msg_type,
            pair: pair.to_string(),
            symbol: symbol.to_string(),
            timestamp: v[0].parse::<i64>().unwrap(),
            snapshot: v[1].parse::<bool>().unwrap(),
            asks,
            bids,
            seq_id,
            prev_seq_id,
            json: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Order, OrderBookMsg, TradeMsg, TradeSide};
    use crypto_market_type::MarketType;
    use crypto_msg_type::MessageType;

    #[test]
    fn test_trade() {
        let trade_msg = TradeMsg {
            exchange: "binance".to_string(),
            market_type: MarketType::LinearSwap,
            symbol: "BTCUSDT".to_string(),
            pair: "BTC/USDT".to_string(),
            msg_type: MessageType::Trade,
            timestamp: 1646092800027,
            side: TradeSide::Sell,
            price: 43150.8,
            quantity_base: 0.001,
            quantity_quote: 43.1508,
            quantity_contract: Some(0.001),
            trade_id: "1108933367".to_string(),
            json: r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1646092800098,"a":1108933367,"s":"BTCUSDT","p":"43150.80","q":"0.001","f":1987119093,"l":1987119093,"T":1646092800027,"m":true}}"#.to_string(),
        };
        let csv_string = trade_msg.to_csv_string();
        let csv_string_expected = r#"1646092800027	sell	43150.8	0.001	43.1508	0.001	1108933367	{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1646092800098,"a":1108933367,"s":"BTCUSDT","p":"43150.80","q":"0.001","f":1987119093,"l":1987119093,"T":1646092800027,"m":true}}"#;
        assert_eq!(csv_string_expected, csv_string);

        let trade_msg_restored = TradeMsg::from_csv_string(
            "binance",
            "linear_swap",
            "trade",
            "BTC/USDT",
            "BTCUSDT",
            &csv_string,
        );
        assert_eq!(
            serde_json::to_string(&trade_msg).unwrap(),
            serde_json::to_string(&trade_msg_restored).unwrap()
        );
    }

    #[test]
    fn test_l2_event() {
        let orderbook_msg = OrderBookMsg {
            exchange: "binance".to_string(),
            market_type: MarketType::LinearSwap,
            symbol: "BTCUSDT".to_string(),
            pair: "BTC/USDT".to_string(),
            msg_type: MessageType::L2Event,
            timestamp: 1648785270714,
            snapshot: false,
            asks: vec![
                Order {
                    price: 44405.4,
                    quantity_base: 0.0,
                    quantity_quote: 0.0,
                    quantity_contract: Some(0.0),
                },
                Order {
                    price: 44427.2,
                    quantity_base: 0.0,
                    quantity_quote: 0.0,
                    quantity_contract: Some(0.0),
                },
            ],
            bids: vec![
                Order {
                    price: 43633.4,
                    quantity_base: 4.515,
                    quantity_quote: 197004.801,
                    quantity_contract: Some(4.515),
                },
                Order {
                    price: 43855.6,
                    quantity_base: 6.058,
                    quantity_quote: 265677.2248,
                    quantity_contract: Some(6.058),
                },
            ],
            seq_id: Some(1343268964711_u64),
            prev_seq_id: Some(1343268961876_u64),
            json: "".to_string(),
        };
        let csv_string = orderbook_msg.to_csv_string();
        let csv_string_expected = r#"1648785270714	false	[[44405.4,0.0,0.0,0.0],[44427.2,0.0,0.0,0.0]]	[[43633.4,4.515,197004.801,4.515],[43855.6,6.058,265677.2248,6.058]]	1343268964711	1343268961876"#;
        assert_eq!(csv_string_expected, csv_string);

        let orderbook_msg_restored = OrderBookMsg::from_csv_string(
            "binance",
            "linear_swap",
            "l2_event",
            "BTC/USDT",
            "BTCUSDT",
            &csv_string,
        );
        assert_eq!(
            serde_json::to_string(&orderbook_msg).unwrap(),
            serde_json::to_string(&orderbook_msg_restored).unwrap()
        );
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

// TradeMsg
impl PartialEq for TradeMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.trade_id == other.trade_id
            && self.price == other.price
            && self.quantity_base == other.quantity_base
            && self.quantity_quote == other.quantity_quote
            && self.quantity_contract == other.quantity_contract
    }
}

impl Eq for TradeMsg {}

impl_partial_ord!(TradeMsg);

impl Ord for TradeMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ret = self.timestamp.cmp(&other.timestamp);
        if ret == std::cmp::Ordering::Equal {
            self.trade_id.cmp(&other.trade_id)
        } else {
            ret
        }
    }
}

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

impl Ord for OrderBookMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ret = self.timestamp.cmp(&other.timestamp);
        if ret == std::cmp::Ordering::Equal {
            if self.seq_id.is_some() && other.seq_id.is_some() {
                self.seq_id.unwrap().cmp(&(other.seq_id.unwrap()))
            } else {
                ret
            }
        } else {
            ret
        }
    }
}

// BboMsg
impl PartialEq for BboMsg {
    fn eq(&self, other: &Self) -> bool {
        self.exchange == other.exchange
            && self.market_type == other.market_type
            && self.symbol == other.symbol
            && self.timestamp == other.timestamp
            && self.bid_price == other.bid_price
            && self.bid_quantity_base == other.bid_quantity_base
            && self.bid_quantity_quote == other.bid_quantity_quote
            && self.ask_price == other.ask_price
            && self.ask_quantity_base == other.ask_quantity_base
            && self.ask_quantity_quote == other.ask_quantity_quote
            && self.id == other.id
    }
}

impl Eq for BboMsg {}

impl_partial_ord!(BboMsg);

impl Ord for BboMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

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

impl Ord for TickerMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

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

impl Ord for CandlestickMsg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

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
        match self {
            Message::Trade(ref a) => match other {
                Message::Trade(ref b) => Some(a.cmp(b)),
                _ => None,
            },
            Message::L2Event(ref a) => match other {
                Message::L2Event(ref b) => Some(a.cmp(b)),
                _ => None,
            },
            Message::Bbo(ref a) => match other {
                Message::Bbo(ref b) => Some(a.cmp(b)),
                _ => None,
            },
            Message::Ticker(ref a) => match other {
                Message::Ticker(ref b) => Some(a.cmp(b)),
                _ => None,
            },
            Message::Candlestick(ref a) => match other {
                Message::Candlestick(ref b) => Some(a.cmp(b)),
                _ => None,
            },
            Message::FundingRate(ref a) => match other {
                Message::FundingRate(ref b) => Some(a.cmp(b)),
                _ => None,
            },
        }
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Message::Trade(ref a) => match other {
                Message::Trade(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            Message::L2Event(ref a) => match other {
                Message::L2Event(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            Message::Bbo(ref a) => match other {
                Message::Bbo(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            Message::Ticker(ref a) => match other {
                Message::Ticker(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            Message::Candlestick(ref a) => match other {
                Message::Candlestick(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            Message::FundingRate(ref a) => match other {
                Message::FundingRate(ref b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
        }
    }
}
