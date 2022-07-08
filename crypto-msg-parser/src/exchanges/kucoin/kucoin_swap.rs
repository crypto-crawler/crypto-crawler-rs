use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::exchanges::{kucoin::message::WebsocketMsg, utils::calc_quantity_and_volume};
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "kucoin";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    baseCurrency: String,
    multiplier: f64,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResponseMsg {
    code: String,
    data: Vec<SwapMarket>,
}

// https://docs.kucoin.cc/futures/#execution-data
#[derive(Serialize, Deserialize)]
struct ContractTradeMsg {
    symbol: String,
    sequence: i64,
    side: String, // buy, sell
    size: f64,
    price: f64,
    ts: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.kucoin.cc/futures/#level-2-market-data
#[derive(Serialize, Deserialize)]
struct ContractOrderbookMsg {
    sequence: i64,
    change: String, // Price, side, quantity
    timestamp: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<ContractTradeMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "match");
    debug_assert!(ws_msg.topic.starts_with("/contractMarket/execution:"));
    let raw_trade = ws_msg.data;
    let pair = crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap();
    let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        raw_trade.price,
        raw_trade.size,
    );

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.symbol.clone(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.ts / 1000000,
        price: raw_trade.price,
        quantity_base,
        quantity_quote,
        quantity_contract,
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

pub(super) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<ContractOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<ContractOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "level2");
    debug_assert!(ws_msg.topic.starts_with("/contractMarket/level2:"));
    let symbol = ws_msg.topic.split(':').last().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let raw_order: Vec<&str> = ws_msg.data.change.split(',').collect();
    let order: Order = {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity = raw_order[2].parse::<f64>().unwrap();

        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let mut asks: Vec<Order> = Vec::new();
    let mut bids: Vec<Order> = Vec::new();
    if raw_order[1] == "sell" {
        asks.push(order);
    } else {
        bids.push(order);
    }

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: ws_msg.data.timestamp,
        seq_id: Some(ws_msg.data.sequence as u64),
        prev_seq_id: None,
        asks,
        bids,
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

// https://docs.kucoin.com/futures/#message-channel-for-the-5-best-ask-bid-full-data-of-level-2
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapL2TopKMsg {
    sequence: u64,
    timestamp: i64,
    asks: Vec<[f64; 2]>,
    bids: Vec<[f64; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SwapL2TopKMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SwapL2TopKMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.subject, "level2");
    debug_assert!(ws_msg.topic.starts_with("/contractMarket/level2Depth"));
    let symbol = ws_msg.topic.split(':').last().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let parse_order = |raw_order: &[f64; 2], pair: &str| -> Order {
        let price = raw_order[0];
        let quantity = raw_order[1];

        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.clone(),
        msg_type: MessageType::L2TopK,
        timestamp: ws_msg.data.timestamp,
        seq_id: Some(ws_msg.data.sequence),
        prev_seq_id: None,
        asks: ws_msg
            .data
            .asks
            .iter()
            .map(|raw_order| parse_order(raw_order, pair.as_str()))
            .collect(),
        bids: ws_msg
            .data
            .bids
            .iter()
            .map(|raw_order| parse_order(raw_order, pair.as_str()))
            .collect(),
        snapshot: true,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
