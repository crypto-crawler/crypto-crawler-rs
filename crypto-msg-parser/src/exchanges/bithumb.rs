use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bithumb";

// see https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md#tradethe-last-spot-trade-msg
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    p: String,
    s: String, // sell, buy
    symbol: String,
    t: String,
    v: String,
    ver: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md#orderbook-the-last-spot-order-book-changed-data
#[derive(Serialize, Deserialize)]
struct SpotOrderbookMsg {
    b: Vec<[String; 2]>,
    s: Vec<[String; 2]>,
    symbol: String,
    ver: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    code: String,
    data: T,
    timestamp: i64,
    topic: String,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    #[derive(Serialize, Deserialize)]
    struct RawMsg {
        code: String,
        data: Value,
        timestamp: i64,
    }
    let raw_msg = serde_json::from_str::<RawMsg>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to RawMsg", msg)))?;
    if raw_msg.data.is_object() {
        Ok(raw_msg.data["symbol"].as_str().unwrap().to_string())
    } else if raw_msg.data.is_array() {
        let arr = raw_msg.data.as_array().unwrap();
        let symbol = arr
            .iter()
            .map(|v| v["symbol"].as_str().unwrap())
            .next()
            .unwrap();
        Ok(symbol.to_string())
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    Ok(obj.get("timestamp").map(|x| x.as_i64().unwrap()))
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    let raw_trades = if ws_msg.code == "00006" {
        // snapshot
        let ws_msg =
            serde_json::from_str::<WebsocketMsg<Vec<SpotTradeMsg>>>(msg).map_err(|_e| {
                SimpleError::new(format!(
                    "Failed to deserialize {} to WebsocketMsg<Vec<SpotTradeMsg>>",
                    msg
                ))
            })?;
        ws_msg.data
    } else if ws_msg.code == "00007" {
        // updates
        let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SpotTradeMsg>",
                msg
            ))
        })?;
        vec![ws_msg.data]
    } else {
        return Err(SimpleError::new(format!("Invalid trade msg {}", msg)));
    };
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.p.parse::<f64>().unwrap();
            let quantity = raw_trade.v.parse::<f64>().unwrap();
            let timestamp = raw_trade.t.parse::<i64>().unwrap() * 1000;
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.symbol.to_string(),
                pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
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
                trade_id: raw_trade.ver.clone(),
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
    debug_assert_eq!(ws_msg.topic, "ORDERBOOK");
    let snapshot = if ws_msg.code == "00006" {
        true
    } else if ws_msg.code == "00007" {
        false
    } else {
        return Err(SimpleError::new(format!("Unknown code {}", ws_msg.code)));
    };
    let symbol = ws_msg.data.symbol;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let timestamp = ws_msg.timestamp;

    let parse_order = |raw_order: &[String; 2]| -> Order {
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
        market_type,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        seq_id: ws_msg.data.ver.parse().ok(),
        prev_seq_id: None,
        asks: ws_msg.data.s.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.b.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
