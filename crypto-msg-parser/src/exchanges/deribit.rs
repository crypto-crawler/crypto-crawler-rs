use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{BboMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use crate::exchanges::utils::calc_quantity_and_volume;

const EXCHANGE_NAME: &str = "deribit";

// see https://docs.deribit.com/?javascript#trades-kind-currency-interval
#[derive(Serialize, Deserialize)]
struct RawTradeMsg {
    trade_seq: i64,
    trade_id: String,
    timestamp: i64,
    price: f64,
    instrument_name: String,
    direction: String, // buy, sell
    amount: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://docs.deribit.com/?javascript#book-instrument_name-interval
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    #[serde(rename = "type")]
    type_: Option<String>, // available values: snapshot, change, none if L2TopK
    timestamp: i64,
    instrument_name: String,
    change_id: u64,
    prev_change_id: Option<u64>,
    bids: Vec<Vec<Value>>, // [Value; 3] if L2Event; [f64; 2] if L2TopK
    asks: Vec<Vec<Value>>, // [Value; 3] if L2Event; [f64; 2] if L2TopK
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Params<T: Sized> {
    channel: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    jsonrpc: String,
    method: String,
    params: Params<T>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RestfulResp<T: Sized> {
    jsonrpc: String,
    usIn: i64,
    usOut: i64,
    usDiff: i64,
    testnet: bool,
    result: T,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsgInverseSwap {
    timestamp: u64,
    instrument_name: String,
    best_bid_price: f64,
    best_bid_amount: f64,
    best_ask_price: f64,
    best_ask_amount: f64,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.params.channel.as_str();
        let data = ws_msg.params.data;

        if channel.starts_with("chart.trades.") {
            let symbol = channel.split('.').nth(2).unwrap();
            Ok(symbol.to_string())
        } else if data.is_object() {
            Ok(data["instrument_name"].as_str().unwrap().to_string())
        } else if data.is_array() {
            let arr = data.as_array().unwrap();
            let symbol = arr
                .iter()
                .map(|v| v["instrument_name"].as_str().unwrap())
                .next()
                .unwrap();
            Ok(symbol.to_string())
        } else {
            Err(SimpleError::new(format!(
                "Unknown websocket message format: {}",
                msg
            )))
        }
    } else if let Ok(rest_resp) = serde_json::from_str::<RestfulResp<Value>>(msg) {
        if let Some(json_obj) = rest_resp.result.as_object() {
            Ok(json_obj["instrument_name"].as_str().unwrap().to_string())
        } else if let Some(arr) = rest_resp.result.as_array() {
            // open interest
            debug_assert!(msg.contains("open_interest"));
            #[allow(clippy::comparison_chain)]
            if arr.len() > 1 {
                Ok("ALL".to_string())
            } else if arr.len() == 1 {
                Ok(arr[0]["instrument_name"].as_str().unwrap().to_string())
            } else {
                Ok("NONE".to_string())
            }
        } else {
            Err(SimpleError::new(format!(
                "Unknown HTTP message format: {}",
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
        let channel = ws_msg.params.channel.as_str();
        let data = ws_msg.params.data;
        if channel.starts_with("chart.trades.") {
            Ok(Some(data["tick"].as_i64().unwrap()))
        } else if data.is_object() {
            Ok(Some(data["timestamp"].as_i64().unwrap()))
        } else if data.is_array() {
            let arr = data.as_array().unwrap();
            let timestamp = arr.iter().map(|x| x["timestamp"].as_i64().unwrap()).max();

            if timestamp.is_none() {
                Err(SimpleError::new(format!("data is empty in {}", msg)))
            } else {
                Ok(timestamp)
            }
        } else {
            Err(SimpleError::new(format!(
                "Unsupported websocket message format: {}",
                msg
            )))
        }
    } else if let Ok(rest_resp) = serde_json::from_str::<RestfulResp<Value>>(msg) {
        if let Some(json_obj) = rest_resp.result.as_object() {
            Ok(Some(json_obj["timestamp"].as_i64().unwrap()))
        } else if let Some(arr) = rest_resp.result.as_array() {
            // open interest
            debug_assert!(msg.contains("open_interest"));
            let timestamp = arr
                .iter()
                .map(|x| x["creation_timestamp"].as_i64().unwrap())
                .max();
            Ok(timestamp)
        } else {
            Err(SimpleError::new(format!(
                "Unknown HTTP message format: {}",
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

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = {
            let arr = ws_msg.params.channel.split('.').collect::<Vec<&str>>();
            arr[0]
        };
        if channel == "trades" {
            MessageType::Trade
        } else if channel == "book" {
            if ws_msg.params.channel.matches('.').count() == 2 {
                MessageType::L2Event
            } else {
                MessageType::L2TopK
            }
        } else if channel == "quote" {
            MessageType::BBO
        } else if channel == "ticker" {
            MessageType::Ticker
        } else if channel == "chart" {
            MessageType::Candlestick
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
    let mut trades: Vec<TradeMsg> = ws_msg
        .params
        .data
        .into_iter()
        .map(|raw_trade| {
            let pair =
                crypto_pair::normalize_pair(&raw_trade.instrument_name, EXCHANGE_NAME).unwrap();
            let (quantity_base, quantity_quote, quantity_contract) = calc_quantity_and_volume(
                EXCHANGE_NAME,
                market_type,
                &pair,
                raw_trade.price,
                raw_trade.amount,
            );

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_name.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp,
                price: raw_trade.price,
                quantity_base,
                quantity_quote,
                quantity_contract,
                side: if raw_trade.direction == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trade_id.to_string(),
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
    debug_assert!(ws_msg.params.channel.starts_with("book."));
    let msg_type = if ws_msg.params.channel.matches('.').count() == 2 {
        MessageType::L2Event
    } else {
        MessageType::L2TopK
    };
    let raw_orderbook = ws_msg.params.data;
    let snapshot = if msg_type == MessageType::L2Event {
        raw_orderbook.type_.unwrap() == "snapshot"
    } else {
        true
    };
    let timestamp = raw_orderbook.timestamp;
    let symbol = raw_orderbook.instrument_name;
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let parse_order = |raw_order: &[Value]| -> Order {
        let (price, quantity) = if raw_order.len() == 3 {
            (
                raw_order[1].as_f64().unwrap(),
                raw_order[2].as_f64().unwrap(),
            )
        } else {
            debug_assert_eq!(2, raw_order.len());
            (
                raw_order[0].as_f64().unwrap(),
                raw_order[1].as_f64().unwrap(),
            )
        };
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
        symbol,
        pair: pair.clone(),
        msg_type,
        timestamp,
        seq_id: Some(raw_orderbook.change_id),
        prev_seq_id: raw_orderbook.prev_change_id,
        asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
        bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    parse_l2(market_type, msg)
}


pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    match market_type {
        MarketType::EuropeanOption => Err(SimpleError::new("Not implemented")),
        MarketType::InverseSwap => parse_bbo_inverse_swap(market_type, msg, received_at),
        _ => unimplemented!()
    }
}


fn parse_bbo_inverse_swap(
    market_type: MarketType,
    msg: &str,
    _received_at: Option<i64>
) -> Result<BboMsg, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsgInverseSwap>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;

    debug_assert!(ws_msg.params.channel.starts_with("quote"));
    let timestamp = extract_timestamp(market_type, msg).unwrap().unwrap();

    let symbol = &ws_msg.params.data.instrument_name.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.params.data.best_ask_price,
        ws_msg.params.data.best_ask_amount
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.params.data.best_bid_price,
        ws_msg.params.data.best_bid_amount
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.params.data.best_ask_price,
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.params.data.best_bid_price,
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: None,
        json: msg.to_string(),
    };

    Ok(bbo_msg)

}