use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::exchanges::utils::calc_quantity_and_volume;
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kraken";

// https://support.kraken.com/hc/en-us/articles/360022839771-Trade
#[derive(Serialize, Deserialize)]
struct Trade {
    feed: String,
    product_id: String,
    side: String, // sell, buy
    uid: String,
    #[serde(rename = "type")]
    type_: String, // fill, liquidation, assignment, termination
    seq: i64,
    time: i64,
    qty: f64,
    price: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct TradeSnapshot {
    feed: String,
    product_id: String,
    trades: Vec<Trade>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://support.kraken.com/hc/en-us/articles/360022635912-Book
#[derive(Serialize, Deserialize)]
struct RawOrder {
    price: f64,
    qty: f64,
}
#[derive(Serialize, Deserialize)]
struct OrderbookSnapshot {
    feed: String,
    product_id: String,
    timestamp: i64,
    seq: i64,
    asks: Vec<RawOrder>,
    bids: Vec<RawOrder>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct OrderbookUpdate {
    feed: String,
    product_id: String,
    side: String, // sell, buy
    seq: i64,
    price: f64,
    qty: f64,
    timestamp: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    if obj.contains_key("product_id") {
        Ok(obj.get("product_id").unwrap().as_str().unwrap().to_string())
    } else if obj.contains_key("serverTime") && obj.contains_key("result") {
        // RESTful API
        if obj["result"].as_str().unwrap() != "success" {
            Err(SimpleError::new(format!("Error HTTP response {}", msg)))
        } else if obj.contains_key("orderBook") {
            Ok("NONE".to_string())
        } else {
            Err(SimpleError::new(format!(
                "Unsupported HTTP message {}",
                msg
            )))
        }
    } else {
        Err(SimpleError::new(format!("No product_id found in {}", msg)))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if obj.contains_key("serverTime") && obj.contains_key("result") {
        // RESTful API
        return Ok(Some(
            DateTime::parse_from_rfc3339(obj["serverTime"].as_str().unwrap())
                .unwrap()
                .timestamp_millis(),
        ));
    }
    let feed = obj["feed"].as_str().unwrap();
    match feed {
        "trade" | "ticker" => Ok(Some(obj["time"].as_i64().unwrap())),
        "trade_snapshot" => {
            let trades = obj["trades"].as_array().unwrap();
            let timestamp = trades
                .iter()
                .map(|raw_trade| raw_trade["time"].as_i64().unwrap())
                .max();
            if timestamp.is_none() {
                Err(SimpleError::new(format!("trades is empty in {}", msg)))
            } else {
                Ok(timestamp)
            }
        }
        "book" | "book_snapshot" => Ok(Some(obj["timestamp"].as_i64().unwrap())),
        _ => Err(SimpleError::new(format!("Unknown feed in {}", msg))),
    }
}

fn convert_trade(raw_trade: Trade) -> TradeMsg {
    let market_type = if raw_trade.product_id.starts_with("PI_") {
        MarketType::InverseSwap
    } else if raw_trade.product_id.starts_with("FI_") {
        MarketType::InverseFuture
    } else {
        MarketType::Unknown
    };
    let pair = crypto_pair::normalize_pair(&raw_trade.product_id, EXCHANGE_NAME).unwrap();
    TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.product_id.clone(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.time,
        price: raw_trade.price,
        quantity_base: raw_trade.qty / raw_trade.price,
        quantity_quote: raw_trade.qty,
        quantity_contract: Some(raw_trade.qty),
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.seq.to_string(),
        json: serde_json::to_string(&raw_trade).unwrap(),
    }
}

pub(crate) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    if let Ok(trade) = serde_json::from_str::<Trade>(msg) {
        Ok(vec![convert_trade(trade)])
    } else if let Ok(trade_snapshot) = serde_json::from_str::<TradeSnapshot>(msg) {
        Ok(trade_snapshot
            .trades
            .into_iter()
            .map(convert_trade)
            .collect())
    } else {
        Err(SimpleError::new(format!("Failed to parse {}", msg)))
    }
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if let Ok(orderbook_update) = serde_json::from_str::<OrderbookUpdate>(msg) {
        let market_type = if orderbook_update.product_id.starts_with("PI_") {
            MarketType::InverseSwap
        } else if orderbook_update.product_id.starts_with("FI_") {
            MarketType::InverseFuture
        } else {
            MarketType::Unknown
        };
        let pair =
            crypto_pair::normalize_pair(&orderbook_update.product_id, EXCHANGE_NAME).unwrap();
        let orders = {
            let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                orderbook_update.price,
                orderbook_update.qty,
            );
            vec![Order {
                price: orderbook_update.price,
                quantity_base,
                quantity_quote,
                quantity_contract,
            }]
        };
        let mut orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: orderbook_update.product_id.clone(),
            pair,
            msg_type: MessageType::L2Event,
            timestamp: orderbook_update.timestamp,
            seq_id: Some(orderbook_update.seq as u64),
            prev_seq_id: None,
            asks: Vec::new(),
            bids: Vec::new(),
            snapshot: false,
            json: msg.to_string(),
        };
        if orderbook_update.side == "buy" {
            orderbook.bids = orders;
        } else if orderbook_update.side == "sell" {
            orderbook.asks = orders;
        } else {
            panic!("Unexpected side {}", orderbook_update.side);
        }
        Ok(vec![orderbook])
    } else if let Ok(orderbook_snapshot) = serde_json::from_str::<OrderbookSnapshot>(msg) {
        let market_type = if orderbook_snapshot.product_id.starts_with("PI_") {
            MarketType::InverseSwap
        } else if orderbook_snapshot.product_id.starts_with("FI_") {
            MarketType::InverseFuture
        } else {
            MarketType::Unknown
        };
        let pair =
            crypto_pair::normalize_pair(&orderbook_snapshot.product_id, EXCHANGE_NAME).unwrap();
        let parse_order = |raw_order: &RawOrder| -> Order {
            let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                raw_order.price,
                raw_order.qty,
            );
            Order {
                price: raw_order.price,
                quantity_base,
                quantity_quote,
                quantity_contract,
            }
        };

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol: orderbook_snapshot.product_id.clone(),
            pair: pair.clone(),
            msg_type: MessageType::L2Event,
            timestamp: orderbook_snapshot.timestamp,
            seq_id: Some(orderbook_snapshot.seq as u64),
            prev_seq_id: None,
            asks: orderbook_snapshot
                .asks
                .iter()
                .map(|raw_order| parse_order(raw_order))
                .collect::<Vec<Order>>(),
            bids: orderbook_snapshot
                .bids
                .iter()
                .map(|raw_order| parse_order(raw_order))
                .collect::<Vec<Order>>(),
            snapshot: true,
            json: msg.to_string(),
        };
        Ok(vec![orderbook])
    } else {
        Err(SimpleError::new(format!("Failed to parse {}", msg)))
    }
}
