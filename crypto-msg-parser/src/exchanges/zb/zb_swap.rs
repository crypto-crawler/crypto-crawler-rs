use crypto_market_type::MarketType;
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};
use crypto_msg_type::MessageType;

use super::EXCHANGE_NAME;
use crate::exchanges::utils::calc_quantity_and_volume;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

pub(super) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if let Some(raw_channel) = obj.get("channel") {
        // websocket
        let raw_channel = raw_channel.as_str().unwrap();
        let channel = raw_channel.split('.').nth(1).unwrap();
        match channel {
            "Trade" => {
                let timestamp = obj["data"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x[3].as_i64().unwrap())
                    .max();
                if let Some(timestamp) = timestamp {
                    Ok(Some(timestamp * 1000))
                } else {
                    Err(SimpleError::new(format!("data is empty in {}", msg)))
                }
            }
            "Depth" | "DepthWhole" => Ok(obj["data"]
                .get("time")
                .map(|x| x.as_str().unwrap().parse::<i64>().unwrap())),
            "Ticker" => {
                if raw_channel == "All.Ticker" {
                    let m = obj["data"].as_object().unwrap();
                    let timestamp = m.values().map(|x| x[6].as_i64().unwrap()).max();
                    if let Some(timestamp) = timestamp {
                        Ok(Some(timestamp * 1000))
                    } else {
                        Err(SimpleError::new(format!("data is empty in {}", msg)))
                    }
                } else {
                    let timestamp = obj["data"].as_array().unwrap()[6].as_i64().unwrap();
                    Ok(Some(timestamp * 1000))
                }
            }
            _ => {
                if channel.starts_with("KLine_") {
                    let timestamp = obj["data"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_array().unwrap()[5].as_i64().unwrap())
                        .max();
                    if let Some(timestamp) = timestamp {
                        Ok(Some(timestamp * 1000))
                    } else {
                        Err(SimpleError::new(format!("data is empty in {}", msg)))
                    }
                } else {
                    Err(SimpleError::new(format!(
                        "Failed to extract timestamp from {}",
                        msg
                    )))
                }
            }
        }
    } else if obj.contains_key("code") && obj.contains_key("desc") && obj.contains_key("data") {
        // ZB linear_swap RESTful
        Ok(obj["data"].get("time").map(|x| x.as_i64().unwrap()))
    } else {
        Err(SimpleError::new(format!("Unknown message format {}", msg)))
    }
}

// doc: https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#86-trade
pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<[f64; 4]>>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Vec<[f64; 4]>>",
            msg
        ))
    })?;
    debug_assert!(ws_msg.channel.ends_with(".Trade"));
    let symbol = ws_msg.channel.split('.').next().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade[0];
            let quantity = raw_trade[1];
            let timestamp = (raw_trade[3] as i64) * 1000;

            let (quantity_base, quantity_quote, quantity_contract) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.to_string(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base,
                quantity_quote,
                quantity_contract,
                side: if raw_trade[3] < 0.0 {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: timestamp.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    data: T,
    #[serde(rename = "type")]
    type_: Option<String>, // Whole
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Level2Msg {
    time: String,
    #[serde(default)]
    asks: Vec<[f64; 2]>,
    #[serde(default)]
    bids: Vec<[f64; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// Docs:
/// * https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#84-increment-depth
/// * https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#84-increment-depth
pub(super) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Level2Msg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Level2Msg>",
            msg
        ))
    })?;
    let msg_type = if ws_msg.channel.ends_with(".DepthWhole") {
        MessageType::L2TopK
    } else if ws_msg.channel.ends_with(".Depth") {
        MessageType::L2Event
    } else {
        panic!("Unsupported channel: {}", ws_msg.channel);
    };
    let snapshot = if msg_type == MessageType::L2TopK {
        true
    } else if let Some(x) = ws_msg.type_ {
        x == "Whole"
    } else {
        false
    };

    let symbol = ws_msg.channel.split('.').next().unwrap();
    let timestamp = ws_msg.data.time.parse::<i64>().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

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
        pair: pair.clone(),
        msg_type,
        timestamp,
        seq_id: None,
        prev_seq_id: None,
        asks: ws_msg
            .data
            .asks
            .iter()
            .map(parse_order)
            .collect::<Vec<Order>>(),
        bids: ws_msg
            .data
            .bids
            .iter()
            .map(parse_order)
            .collect::<Vec<Order>>(),
        snapshot,
        json: msg.to_string(),
    };
    Ok(vec![orderbook])
}
