use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kraken";

// https://docs.kraken.com/websockets/#message-trade
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    #[serde(rename = "type")]
    type_: String,
    trade_id: i64,
    sequence: i64,
    maker_order_id: String,
    taker_order_id: String,
    time: String,
    product_id: String,
    size: String,
    price: String,
    side: String, // b, s
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.kraken.com/websockets/#message-book
#[derive(Serialize, Deserialize)]
struct OrderbookSnapshot {
    #[serde(rename = "as")]
    asks: Vec<[String; 3]>,
    #[serde(rename = "bs")]
    bids: Vec<[String; 3]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.kraken.com/websockets/#message-book
#[derive(Serialize, Deserialize)]
struct OrderbookUpdate {
    a: Option<Vec<Vec<String>>>,
    b: Option<Vec<Vec<String>>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RestResp {
    error: Vec<Value>,
    result: HashMap<String, Value>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(rest_resp) = serde_json::from_str::<RestResp>(msg) {
        // RESTful API
        if !rest_resp.error.is_empty() {
            Err(SimpleError::new(format!("Error http response {}", msg)))
        } else if rest_resp.result.len() > 1 {
            Ok("ALL".to_string())
        } else {
            Ok(rest_resp.result.keys().next().unwrap().to_string())
        }
    } else {
        // websocket
        let arr = serde_json::from_str::<Vec<Value>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg))
        })?;
        let symbol = arr[arr.len() - 1].as_str().unwrap();
        Ok(symbol.to_string())
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if msg.contains("error") && msg.contains("result") {
        // RESTful API
        return Ok(None);
    }
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    debug_assert_eq!(arr.len(), 4);
    let channel = arr[arr.len() - 2].as_str().unwrap();
    if channel == "trade" {
        let raw_trades: Vec<Vec<String>> =
            serde_json::from_value(arr[1].clone()).map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to Vec<Vec<String>>",
                    arr[1]
                ))
            })?;
        Ok(Some(
            (raw_trades[0][2].parse::<f64>().unwrap() * 1000.0) as i64,
        ))
    } else if channel == "spread" {
        let arr: Vec<String> = serde_json::from_value(arr[1].clone()).map_err(SimpleError::from)?;
        Ok(Some((arr[2].parse::<f64>().unwrap() * 1000.0) as i64))
    } else if channel == "ticker" {
        Ok(None)
    } else if channel.starts_with("ohlc-") {
        let timestamp = (arr[1][0].as_str().unwrap().parse::<f64>().unwrap() * 1000.0) as i64;
        Ok(Some(timestamp))
    } else if channel.starts_with("book-") {
        let snapshot = {
            let obj = arr[1].as_object().unwrap();
            obj.contains_key("as") || obj.contains_key("bs")
        };
        if snapshot {
            let orderbook_snapshot = serde_json::from_value::<OrderbookSnapshot>(arr[1].clone())
                .map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to OrderbookSnapshot",
                        arr[1]
                    ))
                })?;
            let mut timestamp = std::i64::MIN;
            {
                for ask in orderbook_snapshot.asks.iter() {
                    let t = (ask[2].parse::<f64>().unwrap() * 1000.0) as i64;
                    if t > timestamp {
                        timestamp = t;
                    }
                }
                for bid in orderbook_snapshot.bids.iter() {
                    let t = (bid[2].parse::<f64>().unwrap() * 1000.0) as i64;
                    if t > timestamp {
                        timestamp = t;
                    }
                }
            };
            if timestamp == std::i64::MIN {
                // as and bs can be both empty, e.g., [6960,{"as":[],"bs":[]},"book-25","LINK/JPY"]
                Ok(None)
            } else {
                Ok(Some(timestamp))
            }
        } else {
            let mut timestamp = std::i64::MIN;
            let mut process_update = |update: OrderbookUpdate| {
                if let Some(a) = update.a {
                    for raw_order in a.iter() {
                        let t = (raw_order[2].parse::<f64>().unwrap() * 1000.0) as i64;
                        if t > timestamp {
                            timestamp = t;
                        }
                    }
                }
                if let Some(b) = update.b {
                    for raw_order in b.iter() {
                        let t = (raw_order[2].parse::<f64>().unwrap() * 1000.0) as i64;
                        if t > timestamp {
                            timestamp = t;
                        }
                    }
                }
            };
            if arr.len() == 4 {
                let update =
                    serde_json::from_value::<OrderbookUpdate>(arr[1].clone()).map_err(|_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to OrderbookUpdate",
                            arr[1]
                        ))
                    })?;
                process_update(update);
            } else if arr.len() == 5 {
                let update =
                    serde_json::from_value::<OrderbookUpdate>(arr[1].clone()).map_err(|_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to OrderbookUpdate",
                            arr[1]
                        ))
                    })?;
                process_update(update);
                let update =
                    serde_json::from_value::<OrderbookUpdate>(arr[2].clone()).map_err(|_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to OrderbookUpdate",
                            arr[2]
                        ))
                    })?;
                process_update(update);
            } else {
                return Err(SimpleError::new(format!("Unknown message format {}", msg)));
            };

            if timestamp == std::i64::MIN {
                Err(SimpleError::new(format!(
                    "Neither a nor b exists in {}",
                    msg
                )))
            } else {
                Ok(Some(timestamp))
            }
        }
    } else {
        Err(SimpleError::new(format!("Unknown channel: {}", channel)))
    }
}

pub(crate) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    debug_assert_eq!(arr[2].as_str().unwrap(), "trade");
    debug_assert_eq!(arr.len(), 4);
    let symbol = arr[arr.len() - 1].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let raw_trades: Vec<Vec<String>> = serde_json::from_value(arr[1].clone()).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to Vec<Vec<String>>",
            arr[1]
        ))
    })?;

    // trade format https://docs.kraken.com/websockets/#message-trade
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade[0].parse::<f64>().unwrap();
            let quantity = raw_trade[1].parse::<f64>().unwrap();
            let timestamp = (raw_trade[2].parse::<f64>().unwrap() * 1000.0) as i64;

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side: if raw_trade[3] == "s" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: timestamp.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg)))?;
    debug_assert_eq!(arr[arr.len() - 2].as_str().unwrap(), "book-25");
    let symbol = arr[arr.len() - 1].as_str().unwrap().to_string();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let snapshot = {
        let obj = arr[1].as_object().unwrap();
        obj.contains_key("as") || obj.contains_key("bs")
    };

    let parse_order = |raw_order: &[String]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();

        Order {
            price,
            quantity_base,
            quantity_quote: price * quantity_base,
            quantity_contract: None,
        }
    };

    let orderbooks = if snapshot {
        let orderbook_snapshot = serde_json::from_value::<OrderbookSnapshot>(arr[1].clone())
            .map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to OrderbookSnapshot",
                    arr[1]
                ))
            })?;

        let timestamp = {
            let mut timestamps: Vec<i64> = Vec::new();
            for ask in orderbook_snapshot.asks.iter() {
                let t = (ask[2].parse::<f64>().unwrap() * 1000.0) as i64;
                timestamps.push(t);
            }
            for bid in orderbook_snapshot.bids.iter() {
                let t = (bid[2].parse::<f64>().unwrap() * 1000.0) as i64;
                timestamps.push(t);
            }
            if timestamps.is_empty() {
                None
            } else {
                let max = *timestamps.iter().max().unwrap();
                Some(max)
            }
        };

        if let Some(timestamp) = timestamp {
            vec![OrderBookMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol,
                pair,
                msg_type: MessageType::L2Event,
                timestamp,
                seq_id: None,
                prev_seq_id: None,
                asks: orderbook_snapshot
                    .asks
                    .iter()
                    .map(|x| parse_order(x))
                    .collect(),
                bids: orderbook_snapshot
                    .bids
                    .iter()
                    .map(|x| parse_order(x))
                    .collect(),
                snapshot,
                json: msg.to_string(),
            }]
        } else {
            vec![]
        }
    } else {
        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();
        let mut timestamps: Vec<i64> = Vec::new();
        let mut process_update = |update: OrderbookUpdate| {
            if let Some(a) = update.a {
                for raw_order in a.iter() {
                    let order = parse_order(raw_order);
                    asks.push(order);
                    let t = (raw_order[2].parse::<f64>().unwrap() * 1000.0) as i64;
                    timestamps.push(t);
                }
            }
            if let Some(b) = update.b {
                for raw_order in b.iter() {
                    let order = parse_order(raw_order);
                    bids.push(order);
                    let t = (raw_order[2].parse::<f64>().unwrap() * 1000.0) as i64;
                    timestamps.push(t);
                }
            }
        };
        if arr.len() == 4 {
            let update =
                serde_json::from_value::<OrderbookUpdate>(arr[1].clone()).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to OrderbookUpdate",
                        arr[1]
                    ))
                })?;
            process_update(update);
        } else if arr.len() == 5 {
            let update =
                serde_json::from_value::<OrderbookUpdate>(arr[1].clone()).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to OrderbookUpdate",
                        arr[1]
                    ))
                })?;
            process_update(update);
            let update =
                serde_json::from_value::<OrderbookUpdate>(arr[2].clone()).map_err(|_e| {
                    SimpleError::new(format!(
                        "Failed to deserialize {} to OrderbookUpdate",
                        arr[2]
                    ))
                })?;
            process_update(update);
        } else {
            return Err(SimpleError::new(format!("Unknown message format {}", msg)));
        };

        let timestamp = if timestamps.is_empty() {
            None
        } else {
            let max = *timestamps.iter().max().unwrap();
            Some(max)
        };
        if let Some(timestamp) = timestamp {
            vec![OrderBookMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol,
                pair,
                msg_type: MessageType::L2Event,
                timestamp,
                seq_id: None,
                prev_seq_id: None,
                asks,
                bids,
                snapshot,
                json: msg.to_string(),
            }]
        } else {
            vec![]
        }
    };

    Ok(orderbooks)
}
