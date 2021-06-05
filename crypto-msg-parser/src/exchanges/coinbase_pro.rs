use crypto_market_type::MarketType;

use crate::order::Order;
use crate::{MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
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

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let raw_trade = serde_json::from_str::<SpotTradeMsg>(msg)?;
    let timestamp = DateTime::parse_from_rfc3339(&raw_trade.time).unwrap();
    let price = raw_trade.price.parse::<f64>().unwrap();
    let quantity = raw_trade.size.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.product_id.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.product_id, EXCHANGE_NAME).unwrap(),
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
        raw: serde_json::to_value(&raw_trade).unwrap(),
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

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let snapshot = {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg)?;
        obj.get("type").unwrap().as_str().unwrap() == "snapshot"
    };
    if snapshot {
        let orderbook_snapshot = serde_json::from_str::<OrderbookSnapshotMsg>(msg)?;
        let symbol = orderbook_snapshot.product_id;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp: Utc::now().timestamp_millis(),
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
            raw: serde_json::from_str(msg)?,
        };

        Ok(vec![orderbook])
    } else {
        let orderbook_updates = serde_json::from_str::<OrderbookUpdateMsg>(msg)?;
        let symbol = orderbook_updates.product_id;
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
        let timestamp = DateTime::parse_from_rfc3339(&orderbook_updates.time).unwrap();

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp: timestamp.timestamp_millis(),
            asks: orderbook_updates
                .changes
                .iter()
                .filter(|x| x[0] == "sell")
                .map(|x| parse_change(x))
                .collect(),
            bids: orderbook_updates
                .changes
                .iter()
                .filter(|x| x[0] == "buy")
                .map(|x| parse_change(x))
                .collect(),
            snapshot,
            raw: serde_json::from_str(msg)?,
        };

        Ok(vec![orderbook])
    }
}
