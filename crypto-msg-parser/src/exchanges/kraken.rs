use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

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

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)?;
    let symbol = arr[3].as_str().unwrap();
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

    let orderbook = if snapshot {
        debug_assert_eq!(arr[2].as_str().unwrap(), "book-25");
        let symbol = arr[3].as_str().unwrap();
        let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
        let orderbook_snapshot = serde_json::from_value::<OrderbookSnapshot>(arr[1].clone())?;
        let timestamp = if !orderbook_snapshot.asks.is_empty() {
            (orderbook_snapshot.asks[0][2].parse::<f64>().unwrap() * 1000.0) as i64
        } else if !orderbook_snapshot.bids.is_empty() {
            (orderbook_snapshot.bids[0][2].parse::<f64>().unwrap() * 1000.0) as i64
        } else {
            panic!("{}", msg);
        };

        OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol: symbol.to_string(),
            pair,
            msg_type: MessageType::L2Event,
            timestamp,
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
        }
    } else {
        let symbol = if arr.len() == 4 {
            debug_assert_eq!(arr[2].as_str().unwrap(), "book-25");
            arr[3].as_str().unwrap()
        } else if arr.len() == 5 {
            debug_assert_eq!(arr[3].as_str().unwrap(), "book-25");
            arr[4].as_str().unwrap()
        } else {
            panic!("Unknown format: {}", msg);
        };
        let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

        let orderbook_updates = serde_json::from_value::<OrderbookUpdate>(arr[1].clone())?;
        let timestamp = if orderbook_updates.a.is_some() {
            (orderbook_updates.a.clone().unwrap()[0][2]
                .parse::<f64>()
                .unwrap()
                * 1000.0) as i64
        } else if orderbook_updates.b.is_some() {
            (orderbook_updates.b.clone().unwrap()[0][2]
                .parse::<f64>()
                .unwrap()
                * 1000.0) as i64
        } else {
            panic!("Both a and b are empty");
        };

        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();

        for x in orderbook_updates.a.iter() {
            for raw_order in x.iter() {
                let order = parse_order(raw_order);
                asks.push(order);
            }
        }
        for x in orderbook_updates.b.iter() {
            for raw_order in x.iter() {
                let order = parse_order(raw_order);
                bids.push(order);
            }
        }
        if arr.len() == 5 {
            let orderbook_updates = serde_json::from_value::<OrderbookUpdate>(arr[2].clone())?;
            for x in orderbook_updates.a.iter() {
                for raw_order in x.iter() {
                    let order = parse_order(raw_order);
                    asks.push(order);
                }
            }
            for x in orderbook_updates.b.iter() {
                for raw_order in x.iter() {
                    let order = parse_order(raw_order);
                    bids.push(order);
                }
            }
        }

        OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol: symbol.to_string(),
            pair,
            msg_type: MessageType::L2Event,
            timestamp,
            asks,
            bids,
            snapshot,
            json: msg.to_string(),
        }
    };

    Ok(vec![orderbook])
}
