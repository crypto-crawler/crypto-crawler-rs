use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "gate";

// https://www.gate.io/docs/websocket/index.html#trades-subscription
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    id: i64,
    time: f64,
    price: String,
    amount: String,
    #[serde(rename = "type")]
    type_: String, // buy, sell
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/websocket/index.html#depth-subscription
#[derive(Serialize, Deserialize)]
struct SpotOrderbookMsg {
    asks: Option<Vec<[String; 2]>>,
    bids: Option<Vec<[String; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct SpotWebsocketMsg {
    method: String,
    params: Vec<Value>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg)?;
    let symbol = ws_msg.params[0].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_trades: Vec<SpotTradeMsg> = serde_json::from_value(ws_msg.params[1].clone()).unwrap();

    let trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.price.parse::<f64>().unwrap();
            let quantity = raw_trade.amount.parse::<f64>().unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type: MessageType::Trade,
                timestamp: (raw_trade.time * 1000.0) as i64,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side: if raw_trade.type_ == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.id.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg)?;
    debug_assert_eq!(ws_msg.params.len(), 3);
    let snapshot = ws_msg.params[0].as_bool().unwrap();
    let symbol = ws_msg.params[2].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_orderbook =
        serde_json::from_value::<SpotOrderbookMsg>(ws_msg.params[1].clone()).unwrap();

    let parse_order = |raw_order: &[String; 2]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();
        vec![price, quantity_base, price * quantity_base]
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: Utc::now().timestamp_millis(),
        asks: if let Some(asks) = raw_orderbook.asks {
            asks.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = raw_orderbook.bids {
            bids.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        snapshot,
        raw: serde_json::from_str(msg)?,
    };

    Ok(vec![orderbook])
}
