use chrono::DateTime;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use super::message::{L2SnapshotRawMsg, WebsocketMsg};

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

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.channel.as_str();
        match channel {
            "v3_trades" => {
                let ws_msg =
                    serde_json::from_str::<WebsocketMsg<RawTradesMsg>>(msg).map_err(|_e| {
                        SimpleError::new(format!(
                            "Failed to deserialize {} to WebsocketMsg<RawTradesMsg>",
                            msg
                        ))
                    })?;
                let timestamp = ws_msg
                    .contents
                    .trades
                    .iter()
                    .map(|raw_trade| {
                        DateTime::parse_from_rfc3339(&raw_trade.createdAt)
                            .unwrap()
                            .timestamp_millis()
                    })
                    .max();
                Ok(timestamp) // contents.trades can be an empty array sometimes
            }
            "v3_orderbook" => Ok(None),
            _ => Err(SimpleError::new(format!(
                "Failed to extract timestamp from {}",
                msg
            ))),
        }
    } else if msg.starts_with(r#"{"markets":"#)
        || serde_json::from_str::<L2SnapshotRawMsg>(msg).is_ok()
    {
        // e.g., https://api.dydx.exchange/v3/markets
        Ok(None)
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradesMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawTradesMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.id;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
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
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to WebsocketMsg", msg)))?;
    let symbol = ws_msg.id;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let snapshot = ws_msg.type_ == "subscribed";
    debug_assert_eq!("v3_orderbook", ws_msg.channel);

    let (asks, bids) = if snapshot {
        let ws_msg =
            serde_json::from_str::<WebsocketMsg<RawOrderBookSnapshotMsg>>(msg).map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to WebsocketMsg<RawOrderBookSnapshotMsg>",
                    msg
                ))
            })?;
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
        let ws_msg =
            serde_json::from_str::<WebsocketMsg<RawOrderBookUpdateMsg>>(msg).map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to WebsocketMsg<RawOrderBookUpdateMsg>",
                    msg
                ))
            })?;
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
