use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::exchanges::utils::calc_quantity_and_volume;
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "ftx";

// https://docs.ftx.com/#trades
#[derive(Serialize, Deserialize)]
struct RawTradeMsg {
    id: i64,
    price: f64,
    size: f64,
    side: String, // buy, sell
    liquidation: bool,
    time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.ftx.com/#orderbooks
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    action: String, // partial, update
    bids: Vec<[f64; 2]>,
    asks: Vec<[f64; 2]>,
    time: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    market: String,
    #[serde(rename = "type")]
    type_: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct RestMsg<T: Sized> {
    success: bool,
    result: T,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        Ok(ws_msg.market)
    } else if let Ok(rest_msg) = serde_json::from_str::<RestMsg<Value>>(msg) {
        if !rest_msg.success {
            return Err(SimpleError::new(format!("Error http response {}", msg)));
        }
        if let Some(result) = rest_msg.result.as_object() {
            if result.contains_key("asks") && result.contains_key("bids") {
                Ok("NONE".to_string())
            } else {
                Err(SimpleError::new(format!(
                    "Unsupported message format {}",
                    msg
                )))
            }
        } else if let Some(result) = rest_msg.result.as_array() {
            #[allow(clippy::comparison_chain)]
            if result.len() > 1 {
                Ok("ALL".to_string())
            } else if result.len() == 1 {
                Ok(result[0]["name"].as_str().unwrap().to_string())
            } else {
                Ok("NONE".to_string())
            }
        } else {
            Err(SimpleError::new(format!(
                "Unsupported message format {}",
                msg
            )))
        }
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.channel.as_str();
        match channel {
            "trades" => {
                let timestamp = ws_msg
                    .data
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| {
                        DateTime::parse_from_rfc3339(x["time"].as_str().unwrap())
                            .unwrap()
                            .timestamp_millis()
                    })
                    .max();

                if timestamp.is_none() {
                    Err(SimpleError::new(format!("data is empty in {}", msg)))
                } else {
                    Ok(timestamp)
                }
            }
            "orderbook" | "ticker" => Ok(Some(
                (ws_msg.data["time"].as_f64().unwrap() * 1000.0) as i64,
            )),
            _ => Err(SimpleError::new(format!(
                "unknown channel {} in {}",
                channel, msg
            ))),
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestMsg<Value>>(msg) {
        if !rest_msg.success {
            Err(SimpleError::new(format!("Error http response {}", msg)))
        } else {
            Ok(None)
        }
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.channel;
        if channel == "trades" {
            MessageType::Trade
        } else if channel == "orderbook" {
            MessageType::L2Event
        } else if channel == "ticker" {
            MessageType::Ticker
        } else {
            MessageType::Other
        }
    } else {
        MessageType::Other
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<RawTradeMsg>>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Vec<RawTradeMsg>>",
            msg
        ))
    })?;
    let symbol = ws_msg.market.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let mut trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.time).unwrap();
            let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                raw_trade.price,
                raw_trade.size,
            );
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price: raw_trade.price,
                quantity_base,
                quantity_quote,
                quantity_contract,
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.id.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.channel, "orderbook");
    let symbol = ws_msg.market.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let snapshot = ws_msg.data.action == "partial";
    let timestamp = (ws_msg.data.time * 1000.0) as i64;

    let parse_order = |raw_order: &[f64; 2]| -> Order {
        let price = raw_order[0];
        let quantity = raw_order[1];
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

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
        pair: pair.to_string(),
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
        asks: ws_msg.data.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
