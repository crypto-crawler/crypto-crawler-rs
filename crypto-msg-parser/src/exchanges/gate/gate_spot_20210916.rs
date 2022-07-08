use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;
use super::EXCHANGE_NAME;

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

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg).map_err(|_e| {
        SimpleError::new(format!("Failed to deserialize {} to SpotWebsocketMsg", msg))
    })?;
    if ws_msg.method == "trades.update" {
        Ok(ws_msg.params[0].as_str().unwrap().to_string())
    } else if ws_msg.method == "depth.update" {
        Ok(ws_msg.params[2].as_str().unwrap().to_string())
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg).map_err(|_e| {
        SimpleError::new(format!("Failed to deserialize {} to SpotWebsocketMsg", msg))
    })?;
    if ws_msg.method == "trades.update" {
        let raw_trades = ws_msg.params[1].as_array().unwrap();
        let timestamp = raw_trades
            .iter()
            .map(|raw_trade| (raw_trade.get("time").unwrap().as_f64().unwrap() * 1000.0) as i64)
            .max();
        if timestamp.is_none() {
            Err(SimpleError::new(format!("as and bs are empty in {}", msg)))
        } else {
            Ok(timestamp)
        }
    } else if ws_msg.method == "depth.update" {
        Ok(None)
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

#[deprecated(since = "1.3.7", note = "Gate has new data format since 2021-09-16")]
pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg).map_err(|_e| {
        SimpleError::new(format!("Failed to deserialize {} to SpotWebsocketMsg", msg))
    })?;
    let symbol = ws_msg.params[0].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let raw_trades: Vec<SpotTradeMsg> =
        serde_json::from_value(ws_msg.params[1].clone()).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to Vec<SpotTradeMsg>",
                ws_msg.params[1]
            ))
        })?;

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

#[deprecated(since = "1.3.7", note = "Gate has new data format since 2021-09-16")]
pub(crate) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<SpotWebsocketMsg>(msg).map_err(|_e| {
        SimpleError::new(format!("Failed to deserialize {} to SpotWebsocketMsg", msg))
    })?;
    debug_assert_eq!(ws_msg.params.len(), 3);
    let snapshot = ws_msg.params[0].as_bool().unwrap();
    let symbol = ws_msg.params[2].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let raw_orderbook = serde_json::from_value::<SpotOrderbookMsg>(ws_msg.params[1].clone())
        .map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to SpotOrderbookMsg",
                ws_msg.params[1]
            ))
        })?;

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
