use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "kucoin";

// https://docs.kucoin.com/#match-execution-data
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    symbol: String,
    sequence: String,
    side: String, // buy, sell
    size: String,
    price: String,
    time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Changes {
    asks: Vec<[String; 3]>, //price, size, sequence
    bids: Vec<[String; 3]>,
}
// https://docs.kucoin.com/#level-2-market-data
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookMsg {
    sequenceStart: i64,
    symbol: String,
    changes: Changes,
    sequenceEnd: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg)?;
    debug_assert_eq!(ws_msg.subject, "trade.l3match");
    debug_assert!(ws_msg.topic.starts_with("/market/match:"));
    let raw_trade = ws_msg.data;
    let price = raw_trade.price.parse::<f64>().unwrap();
    let quantity = raw_trade.size.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: raw_trade.symbol.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
        msg_type: MessageType::Trade,
        timestamp: raw_trade.time.parse::<i64>().unwrap() / 1000000,
        price,
        quantity_base: quantity,
        quantity_quote: price * quantity,
        quantity_contract: None,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.sequence.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

pub(crate) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg)?;
    debug_assert_eq!(ws_msg.subject, "trade.l2update");
    debug_assert!(ws_msg.topic.starts_with("/market/level2:"));
    let symbol = ws_msg.data.symbol;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    let parse_order = |raw_order: &[String; 3]| -> Order {
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
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: Some(ws_msg.data.sequenceStart as u64),
        prev_seq_id: None,
        asks: ws_msg
            .data
            .changes
            .asks
            .iter()
            .map(|x| parse_order(x))
            .collect(),
        bids: ws_msg
            .data
            .changes
            .bids
            .iter()
            .map(|x| parse_order(x))
            .collect(),
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
