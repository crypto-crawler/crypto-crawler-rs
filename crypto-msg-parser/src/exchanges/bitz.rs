use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bitz";

// see https://apidocv2.bitz.plus/#order
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotTradeMsg {
    id: String,
    t: String,
    T: i64,
    p: String,
    n: String,
    s: String, // Sell, Buy
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://apidocv2.bitz.plus/#depth
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookMsg {
    asks: Option<Vec<[Value; 3]>>,
    bids: Option<Vec<[Value; 3]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Params {
    symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    params: Params,
    action: String,
    data: T,
    time: i64,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    let symbol = ws_msg.params.symbol.as_str();
    Ok(symbol.to_string())
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    Ok(Some(ws_msg.time))
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SpotTradeMsg>>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotTradeMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.params.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let mut trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.p.parse::<f64>().unwrap();
            let quantity = raw_trade.n.parse::<f64>().unwrap();
            let timestamp = if raw_trade.id.is_empty() {
                raw_trade.T * 1000
            } else {
                raw_trade.id.parse::<i64>().unwrap()
            };
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
                side: if raw_trade.s == "sell" {
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

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.action, "Pushdata.depth");
    let symbol = ws_msg.params.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let parse_order = |raw_order: &[Value; 3]| -> Order {
        let price = raw_order[0].as_str().unwrap().parse::<f64>().unwrap();
        let (quantity_base, quantity_quote) = if raw_order[1].is_i64() {
            (0.0, 0.0)
        } else {
            let base = raw_order[1].as_str().unwrap().parse::<f64>().unwrap();
            let quote = raw_order[2].as_str().unwrap().parse::<f64>().unwrap();
            (base, quote)
        };

        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract: None,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: ws_msg.time,
        seq_id: None,
        prev_seq_id: None,
        asks: if let Some(asks) = ws_msg.data.asks {
            asks.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = ws_msg.data.bids {
            bids.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
