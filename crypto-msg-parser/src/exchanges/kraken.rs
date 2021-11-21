use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
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

pub(crate) fn extract_symbol(_market_type_: MarketType, msg: &str) -> Option<String> {
    let arr = serde_json::from_str::<Vec<Value>>(msg).unwrap();
    let symbol = arr[arr.len() - 1].as_str().unwrap();
    Some(symbol.to_string())
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)?;
    debug_assert_eq!(arr[2].as_str().unwrap(), "trade");
    debug_assert_eq!(arr.len(), 4);
    let symbol = arr[arr.len() - 1].as_str().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_trades: Vec<Vec<String>> = serde_json::from_value(arr[1].clone()).unwrap();

    // trade format https://docs.kraken.com/websockets/#message-trade
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade[0].parse::<f64>().unwrap();
            let quantity = raw_trade[1].parse::<f64>().unwrap();
            let timestamp = (raw_trade[2].parse::<f64>().unwrap() * 1000.0) as i64;

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
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

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    debug_assert_eq!(market_type, MarketType::Spot);
    let arr = serde_json::from_str::<Vec<Value>>(msg)?;
    debug_assert_eq!(arr[arr.len() - 2].as_str().unwrap(), "book-25");
    let symbol = arr[arr.len() - 1].as_str().unwrap().to_string();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let snapshot = arr[1].as_object().unwrap().contains_key("as");

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
        let orderbook_snapshot = serde_json::from_value::<OrderbookSnapshot>(arr[1].clone())?;

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
            let update = serde_json::from_value::<OrderbookUpdate>(arr[1].clone())?;
            process_update(update);
        } else if arr.len() == 5 {
            let update = serde_json::from_value::<OrderbookUpdate>(arr[1].clone())?;
            process_update(update);
            let update = serde_json::from_value::<OrderbookUpdate>(arr[2].clone())?;
            process_update(update);
        } else {
            panic!("Unknown message format {}", msg);
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
