use chrono::DateTime;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "dydx";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    size: String,
    side: String, // BUY, SELL
    price: String,
    createdAt: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradesMsg {
    trades: Vec<RawTradeMsg>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RawOrder {
    size: String,
    price: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RawOrderBookSnapshotMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    asks: Vec<RawOrder>,
    bids: Vec<RawOrder>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RawOrderBookUpdateMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradesMsg>>(msg)?;
    let symbol = ws_msg.id;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    debug_assert_eq!("v3_trades", ws_msg.channel);

    let mut trades: Vec<TradeMsg> = ws_msg
        .contents
        .trades
        .into_iter()
        .map(|raw_trade| {
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.createdAt)
                .unwrap()
                .timestamp_millis();
            let price = raw_trade.price.parse::<f64>().unwrap();
            let size = raw_trade.size.parse::<f64>().unwrap();
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.to_string(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base: size,
                quantity_quote: price * size,
                quantity_contract: Some(size),
                side: if raw_trade.side == "SELL" {
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

fn parse_order_update(raw_order: &[String; 2]) -> Order {
    let price = raw_order[0].parse::<f64>().unwrap();
    let size = raw_order[1].parse::<f64>().unwrap();

    Order {
        price,
        quantity_base: size,
        quantity_quote: price * size,
        quantity_contract: Some(size),
    }
}

fn parse_order_snapshot(raw_order: &RawOrder) -> Order {
    let price = raw_order.price.parse::<f64>().unwrap();
    let size = raw_order.size.parse::<f64>().unwrap();

    Order {
        price,
        quantity_base: size,
        quantity_quote: price * size,
        quantity_contract: Some(size),
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)?;
    let symbol = ws_msg.id;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let snapshot = ws_msg.type_ == "subscribed";
    debug_assert_eq!("v3_orderbook", ws_msg.channel);

    let (asks, bids) = if snapshot {
        let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderBookSnapshotMsg>>(msg)?;
        (
            ws_msg
                .contents
                .asks
                .into_iter()
                .map(|x| parse_order_snapshot(&x))
                .collect(),
            ws_msg
                .contents
                .bids
                .into_iter()
                .map(|x| parse_order_snapshot(&x))
                .collect(),
        )
    } else {
        let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderBookUpdateMsg>>(msg)?;
        (
            ws_msg
                .contents
                .asks
                .into_iter()
                .map(|x| parse_order_update(&x))
                .collect(),
            ws_msg
                .contents
                .bids
                .into_iter()
                .map(|x| parse_order_update(&x))
                .collect(),
        )
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        asks,
        bids,
        seq_id: None,
        prev_seq_id: None,
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
