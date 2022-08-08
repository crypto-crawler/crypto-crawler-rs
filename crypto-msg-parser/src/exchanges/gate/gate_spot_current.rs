use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::messages::WebsocketMsg;
use crypto_message::{BboMsg, Order, OrderBookMsg, TradeMsg, TradeSide};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "gate";

// https://www.gate.io/docs/developers/apiv4/ws/en/#server-notification-2
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    id: i64,
    create_time: f64,
    create_time_ms: String,
    side: String, // buy, sell
    currency_pair: String,
    amount: String,
    price: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/developers/apiv4/ws/en/#changed-order-book-levels
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookUpdateMsg {
    t: i64,
    e: String,
    E: i64,
    s: String,
    U: i64,
    u: i64,
    a: Option<Vec<[String; 2]>>,
    b: Option<Vec<[String; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/developers/apiv4/ws/en/#limited-level-full-order-book-snapshot
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookSnapshotMsg {
    t: i64,
    lastUpdateId: i64,
    s: String,
    asks: Option<Vec<[String; 2]>>,
    bids: Option<Vec<[String; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.gate.io/docs/developers/apiv4/en/#retrieve-order-book
#[derive(Serialize, Deserialize)]
struct SpotRestL2SnapshotMsg {
    current: i64,
    update: i64,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
}

// https://www.gate.io/docs/developers/apiv4/ws/en/#best-bid-or-ask-price
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsg {
    t: i64,    // Order book update time in milliseconds
    u: i64,    // Order book update ID
    s: String, // Currency pair
    b: String, // best bid price
    B: String, // best bid amount
    a: String, // best ask price
    A: String, // best ask amount
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<HashMap<String, Value>>>(msg) {
        if let Some(symbol) = ws_msg.result.get("currency_pair") {
            Ok(symbol.as_str().unwrap().to_string())
        } else if let Some(symbol) = ws_msg.result.get("s") {
            Ok(symbol.as_str().unwrap().to_string())
        } else if let Some(symbol) = ws_msg.result.get("n") {
            let n = symbol.as_str().unwrap();
            let pos = n.find('_').unwrap();
            let symbol = &n[(pos + 1)..];
            Ok(symbol.to_string())
        } else {
            Err(SimpleError::new(format!(
                "Unsupported websocket message format {}",
                msg
            )))
        }
    } else if serde_json::from_str::<SpotRestL2SnapshotMsg>(msg).is_ok() {
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<HashMap<String, Value>>>(msg) {
        if ws_msg.channel == "spot.trades" {
            Ok(Some(
                ws_msg.result["create_time_ms"]
                    .as_str()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap() as i64,
            ))
        } else if ws_msg.channel.starts_with("spot.order_book")
            || ws_msg.channel == "spot.book_ticker"
        {
            Ok(Some(ws_msg.result["t"].as_i64().unwrap()))
        } else {
            Ok(Some(ws_msg.time * 1000))
        }
    } else if let Ok(l2_snapshot) = serde_json::from_str::<SpotRestL2SnapshotMsg>(msg) {
        Ok(Some(l2_snapshot.current))
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotTradeMsg>",
            msg
        ))
    })?;
    debug_assert_eq!(ws_msg.channel, "spot.trades");
    debug_assert_eq!(ws_msg.event, "update");
    let result = ws_msg.result;
    let symbol = result.currency_pair;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let price = result.price.parse::<f64>().unwrap();
    let quantity_base = result.amount.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::Trade,
        timestamp: result.create_time_ms.parse::<f64>().unwrap() as i64,
        price: result.price.parse::<f64>().unwrap(),
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
        side: if result.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: result.id.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

fn parse_order(raw_order: &[String; 2]) -> Order {
    let price = raw_order[0].parse::<f64>().unwrap();
    let quantity_base = raw_order[1].parse::<f64>().unwrap();
    Order {
        price,
        quantity_base,
        quantity_quote: price * quantity_base,
        quantity_contract: None,
    }
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<Value>",
            msg
        ))
    })?;
    if ws_msg.channel == "spot.order_book_update" {
        parse_l2_update(msg)
    } else if ws_msg.channel == "spot.order_book" {
        parse_l2_snapshot(msg)
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

fn parse_l2_update(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<SpotOrderbookUpdateMsg>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SpotOrderbookUpdateMsg>",
                msg
            ))
        })?;
    debug_assert_eq!(ws_msg.channel, "spot.order_book_update");
    let result = ws_msg.result;
    let symbol = result.s;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp: result.t,
        seq_id: Some(result.u as u64),
        prev_seq_id: Some(result.U as u64 - 1),
        asks: if let Some(asks) = result.a {
            asks.iter().map(parse_order).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = result.b {
            bids.iter().map(parse_order).collect()
        } else {
            Vec::new()
        },
        snapshot: ws_msg.event == "all",
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

fn parse_l2_snapshot(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg =
        serde_json::from_str::<WebsocketMsg<SpotOrderbookSnapshotMsg>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to WebsocketMsg<SpotOrderbookSnapshotMsg>",
                msg
            ))
        })?;
    debug_assert_eq!(ws_msg.channel, "spot.order_book");
    debug_assert_eq!(ws_msg.event, "update");
    let result = ws_msg.result;
    let symbol = result.s;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

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
        market_type: MarketType::Spot,
        symbol,
        pair,
        msg_type: MessageType::L2Event,
        timestamp: result.t,
        seq_id: None,
        prev_seq_id: None,
        asks: if let Some(asks) = result.asks {
            asks.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        bids: if let Some(bids) = result.bids {
            bids.iter().map(|x| parse_order(x)).collect()
        } else {
            Vec::new()
        },
        snapshot: true,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(super) fn parse_bbo(msg: &str) -> Result<Vec<BboMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsg>>(msg).map_err(SimpleError::from)?;
    debug_assert_eq!("spot.book_ticker", ws_msg.channel);
    debug_assert_eq!("update", ws_msg.event);

    let symbol = ws_msg.result.s.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let timestamp = ws_msg.result.t;

    let ask_price = ws_msg.result.a.parse::<f64>().unwrap();
    let ask_size = ws_msg.result.A.parse::<f64>().unwrap();

    let bid_price = ws_msg.result.b.parse::<f64>().unwrap();
    let bid_size = ws_msg.result.B.parse::<f64>().unwrap();

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type: MarketType::Spot,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price,
        ask_quantity_base: ask_size,
        ask_quantity_quote: ask_price * ask_size,
        ask_quantity_contract: None,
        bid_price,
        bid_quantity_base: bid_size,
        bid_quantity_quote: bid_price * bid_size,
        bid_quantity_contract: None,
        id: None,
        json: msg.to_string(),
    };

    Ok(vec![bbo_msg])
}
