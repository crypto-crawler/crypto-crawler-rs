use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bithumb";

// see https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md#tradethe-last-spot-trade-msg
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    p: String,
    s: String, // sell, buy
    symbol: String,
    t: String,
    v: String,
    ver: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md#orderbook-the-last-spot-order-book-changed-data
#[derive(Serialize, Deserialize)]
struct SpotOrderbookMsg {
    b: Vec<[String; 2]>,
    s: Vec<[String; 2]>,
    symbol: String,
    ver: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    code: String,
    data: T,
    timestamp: i64,
    topic: String,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    if ws_msg.data.is_object() {
        Some(ws_msg.data["symbol"].as_str().unwrap().to_string())
    } else if ws_msg.data.is_array() {
        let arr = ws_msg.data.as_array().unwrap();
        let symbols = arr
            .iter()
            .map(|v| v["symbol"].as_str().unwrap())
            .collect::<Vec<&str>>();
        Some(symbols[0].to_string())
    } else {
        panic!("Unknown message format: {}", msg);
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)?;
    let raw_trades = if ws_msg.code == "00006" {
        // snapshot
        let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SpotTradeMsg>>>(msg)?;
        ws_msg.data
    } else if ws_msg.code == "00007" {
        // updates
        let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg)?;
        vec![ws_msg.data]
    } else {
        panic!("Invalid trade msg {}", msg);
    };
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.p.parse::<f64>().unwrap();
            let quantity = raw_trade.v.parse::<f64>().unwrap();
            let timestamp = raw_trade.t.parse::<i64>().unwrap() * 1000;
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.symbol.to_string(),
                pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side: if raw_trade.s == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.ver.clone(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();
    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg)?;
    debug_assert_eq!(ws_msg.topic, "ORDERBOOK");
    let snapshot = if ws_msg.code == "00006" {
        true
    } else if ws_msg.code == "00007" {
        false
    } else {
        panic!("Unknown code {}", ws_msg.code);
    };
    let symbol = ws_msg.data.symbol;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let timestamp = ws_msg.timestamp;

    let parse_order = |raw_order: &[String; 2]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();

        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: ws_msg.data.ver.parse().ok(),
        prev_seq_id: None,
        asks: ws_msg.data.s.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.b.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
