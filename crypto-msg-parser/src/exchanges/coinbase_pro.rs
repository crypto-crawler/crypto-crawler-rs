use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "coinbase_pro";

// see https://docs.pro.coinbase.com/#match
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
    side: String, // buy, sell
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://docs.pro.coinbase.com/#the-level2-channel
#[derive(Serialize, Deserialize)]
struct OrderbookSnapshotMsg {
    #[serde(rename = "type")]
    type_: String,
    product_id: String,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://docs.pro.coinbase.com/#the-level2-channel
#[derive(Serialize, Deserialize)]
struct OrderbookUpdateMsg {
    #[serde(rename = "type")]
    type_: String,
    product_id: String,
    time: String,
    changes: Vec<[String; 3]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if let Some(product_id) = json_obj.get("product_id") {
        Ok(product_id.as_str().unwrap().to_string())
    } else if json_obj.contains_key("asks") && json_obj.contains_key("bids") {
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if json_obj.contains_key("type") && json_obj["type"].is_string() {
        let type_ = json_obj["type"].as_str().unwrap();
        if type_ == "snapshot" {
            Ok(None) // orderbook snapshot doesn't have a timestamp
        } else if let Some(time) = json_obj.get("time") {
            let time_str = time.as_str().unwrap();
            if time_str.starts_with("0001-01-01T00:00:00") {
                Ok(None)
            } else {
                Ok(Some(
                    DateTime::parse_from_rfc3339(time_str)
                        .unwrap()
                        .timestamp_millis(),
                ))
            }
        } else {
            Err(SimpleError::new(format!(
                "Failed to extract timestamp from {}",
                msg
            )))
        }
    } else if json_obj.contains_key("asks") && json_obj.contains_key("bids") {
        // l2_snapshot doesn't have a timestamp
        Ok(None)
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        )))
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let raw_trade = serde_json::from_str::<SpotTradeMsg>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to SpotTradeMsg", msg)))?;
    let timestamp = DateTime::parse_from_rfc3339(&raw_trade.time).unwrap();
    let price = raw_trade.price.parse::<f64>().unwrap();
    let quantity = raw_trade.size.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.product_id.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.product_id, EXCHANGE_NAME).ok_or_else(
            || {
                SimpleError::new(format!(
                    "Failed to normalize {} from {}",
                    raw_trade.product_id, msg
                ))
            },
        )?,
        msg_type: MessageType::Trade,
        timestamp: timestamp.timestamp_millis(),
        price,
        quantity_base: quantity,
        quantity_quote: price * quantity,
        quantity_contract: None,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.trade_id.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

fn parse_order(raw_order: &[String; 2]) -> Order {
    let price = raw_order[0].parse::<f64>().unwrap();
    let quantity_base = raw_order[1].parse::<f64>().unwrap();

    Order {
        price,
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
    }
}

fn parse_change(raw_order: &[String; 3]) -> Order {
    let price = raw_order[1].parse::<f64>().unwrap();
    let quantity_base = raw_order[2].parse::<f64>().unwrap();

    Order {
        price,
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let snapshot = {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to HashMap<String, Value>",
                msg
            ))
        })?;
        obj.get("type").unwrap().as_str().unwrap() == "snapshot"
    };
    if snapshot {
        let orderbook_snapshot =
            serde_json::from_str::<OrderbookSnapshotMsg>(msg).map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to OrderbookSnapshotMsg",
                    msg
                ))
            })?;
        let symbol = orderbook_snapshot.product_id;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp: timestamp.expect("Coinbase level2 snapshot messages don't have timestamp"),
            seq_id: None,
            prev_seq_id: None,
            asks: orderbook_snapshot.asks.iter().map(parse_order).collect(),
            bids: orderbook_snapshot.bids.iter().map(parse_order).collect(),
            snapshot,
            json: msg.to_string(),
        };

        Ok(vec![orderbook])
    } else {
        let orderbook_updates = serde_json::from_str::<OrderbookUpdateMsg>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to OrderbookUpdateMsg",
                msg
            ))
        })?;
        let symbol = orderbook_updates.product_id;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;
        let timestamp = DateTime::parse_from_rfc3339(&orderbook_updates.time).unwrap();

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp: timestamp.timestamp_millis(),
            seq_id: None,
            prev_seq_id: None,
            asks: orderbook_updates
                .changes
                .iter()
                .filter(|x| x[0] == "sell")
                .map(parse_change)
                .collect(),
            bids: orderbook_updates
                .changes
                .iter()
                .filter(|x| x[0] == "buy")
                .map(parse_change)
                .collect(),
            snapshot,
            json: msg.to_string(),
        };

        Ok(vec![orderbook])
    }
}
