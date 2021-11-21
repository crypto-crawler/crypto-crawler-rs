use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

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

pub(super) fn extract_symbol(msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg).unwrap();
    if ws_msg.method == "trades.update" {
        Some(ws_msg.params[0].as_str().unwrap().to_string())
    } else if ws_msg.method == "depth.update" {
        Some(ws_msg.params[2].as_str().unwrap().to_string())
    } else {
        panic!("Unsupported message format: {}", msg);
    }
}

#[deprecated(since = "1.3.7", note = "Gate has new data format since 2020-09-16")]
pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg)?;
    let symbol = ws_msg.params[0].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_trades: Vec<SpotTradeMsg> = serde_json::from_value(ws_msg.params[1].clone()).unwrap();

    let mut trades: Vec<TradeMsg> = raw_trades
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
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

#[deprecated(since = "1.3.7", note = "Gate has new data format since 2020-09-16")]
pub(crate) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>> {
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
        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
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
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
