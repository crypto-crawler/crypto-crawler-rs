use crate::order::Order;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

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
    #[serde(with = "crate::f64_limited_serde")]
    pub quantity_base: f64,
    // Number of quote coins(mostly USDT)
    #[serde(with = "crate::f64_limited_serde")]
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

// TSV utilities.

impl TradeMsg {
    /// Convert to a TSV string.
    ///
    /// The `exchange`, `market_type`, `msg_type`, `pair` and `symbol` fields are not
    /// included to save some disk space.
    pub fn to_csv_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.timestamp,
            self.side,
            self.price,
            self.quantity_base,
            self.quantity_quote,
            self.quantity_contract
                .map(|x| x.to_string())
                .unwrap_or_default(),
            self.trade_id,
            self.json
        )
    }

    /// Convert from a TSV string.
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
    /// Convert to a TSV string.
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

    /// Convert from a TSV string.
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
    use super::TradeSide;
    use super::{Order, OrderBookMsg, TradeMsg};
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
        let tsv_string = trade_msg.to_csv_string();
        let tsv_string_expected = r#"1646092800027	sell	43150.8	0.001	43.1508	0.001	1108933367	{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1646092800098,"a":1108933367,"s":"BTCUSDT","p":"43150.80","q":"0.001","f":1987119093,"l":1987119093,"T":1646092800027,"m":true}}"#;
        assert_eq!(tsv_string_expected, tsv_string);

        let trade_msg_restored = TradeMsg::from_csv_string(
            "binance",
            "linear_swap",
            "trade",
            "BTC/USDT",
            "BTCUSDT",
            &tsv_string,
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
        let tsv_string = orderbook_msg.to_csv_string();
        let tsv_string_expected = r#"1648785270714	false	[[44405.4,0.0,0.0,0.0],[44427.2,0.0,0.0,0.0]]	[[43633.4,4.515,197004.801,4.515],[43855.6,6.058,265677.2248,6.058]]	1343268964711	1343268961876"#;
        assert_eq!(tsv_string_expected, tsv_string);

        let orderbook_msg_restored = OrderBookMsg::from_csv_string(
            "binance",
            "linear_swap",
            "l2_event",
            "BTC/USDT",
            "BTCUSDT",
            &tsv_string,
        );
        assert_eq!(
            serde_json::to_string(&orderbook_msg).unwrap(),
            serde_json::to_string(&orderbook_msg_restored).unwrap()
        );
    }
}
