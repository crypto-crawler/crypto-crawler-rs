use crypto_market_type::MarketType;

use crate::{msg::TradeMsg, MessageType};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "binance";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct AggTradeMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    a: i64,    // Aggregate trade ID
    p: String, // Price
    q: String, // Quantity
    f: i64,    // First trade ID
    l: i64,    // Last trade ID
    T: i64,    // Trade time
    m: bool,   // Is the buyer the market maker?
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    t: i64,    // Trade ID
    p: String, // Price
    q: String, // Quantity
    b: i64,    // Bid order ID
    a: i64,    // Ask order ID
    T: i64,    // Trade time
    m: bool,   // Is the buyer the market maker?
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionTradeMsg {
    t: String, // Trade ID
    p: String, // Price
    q: String, // Quantity
    b: String, // Bid order ID
    a: String, // Ask order ID
    T: i64,    // Trade time
    s: String, // Side
    S: String, // Symbol
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionTradeAllMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    t: Vec<OptionTradeMsg>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct WebsocketMsg<T> {
    stream: String,
    data: T,
}

fn calc_quantity(market_type: MarketType, pair: &str, price: f64, quantity: f64) -> f64 {
    if market_type == MarketType::InverseSwap || market_type == MarketType::InverseFuture {
        let contract_value = if pair.starts_with("BTC/") {
            100.0
        } else {
            10.0
        };
        quantity * contract_value / price
    } else {
        quantity
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg)?;
    let data = obj.get("data").unwrap();
    let event_type = data.get("e").unwrap().as_str().unwrap();

    match event_type {
        "aggTrade" => {
            let agg_trade: AggTradeMsg = serde_json::from_value(data.clone()).unwrap();
            let mut trade = TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: agg_trade.s.clone(),
                pair: crypto_pair::normalize_pair(&agg_trade.s, "binance").unwrap(),
                msg_type: MessageType::Trade,
                timestamp: agg_trade.T,
                price: agg_trade.p.parse::<f64>().unwrap(),
                quantity: agg_trade.q.parse::<f64>().unwrap(),
                side: agg_trade.m,
                trade_id: agg_trade.a.to_string(),
                raw: serde_json::from_str(msg)?,
            };
            trade.quantity = calc_quantity(market_type, &trade.pair, trade.price, trade.quantity);
            Ok(vec![trade])
        }
        "trade" => {
            let raw_trade: RawTradeMsg = serde_json::from_value(data.clone()).unwrap();
            let mut trade = TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.s.clone(),
                pair: crypto_pair::normalize_pair(&raw_trade.s, "binance").unwrap(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.T,
                price: raw_trade.p.parse::<f64>().unwrap(),
                quantity: raw_trade.q.parse::<f64>().unwrap(),
                side: raw_trade.m,
                trade_id: raw_trade.t.to_string(),
                raw: serde_json::from_str(msg)?,
            };
            trade.quantity = calc_quantity(market_type, &trade.pair, trade.price, trade.quantity);
            Ok(vec![trade])
        }
        "trade_all" => {
            let all_trades: OptionTradeAllMsg = serde_json::from_value(data.clone()).unwrap();
            let trades: Vec<TradeMsg> = all_trades
                .t
                .into_iter()
                .map(|trade| TradeMsg {
                    exchange: EXCHANGE_NAME.to_string(),
                    market_type,
                    symbol: trade.S.clone(),
                    pair: crypto_pair::normalize_pair(&trade.S, "binance").unwrap(),
                    msg_type: MessageType::Trade,
                    timestamp: trade.T,
                    price: trade.p.parse::<f64>().unwrap(),
                    quantity: trade.q.parse::<f64>().unwrap(),
                    side: trade.s == "1", // TODO: find the meaning after option doc is released
                    trade_id: trade.a.to_string(),
                    raw: serde_json::to_value(trade).unwrap(),
                })
                .collect();

            Ok(trades)
        }
        _ => panic!("Unsupported event type {}", event_type),
    }
}
