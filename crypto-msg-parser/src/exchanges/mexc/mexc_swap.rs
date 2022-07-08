use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::super::utils::calc_quantity_and_volume;
use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

// https://mxcdevelop.github.io/APIDoc/contract.api.cn.html#4483df6e28
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    p: f64, // price
    v: f64, // quantity
    T: i64, // 1, buy; 2, sell
    t: i64, // timestamp
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://mxcdevelop.github.io/APIDoc/contract.api.cn.html#a1128a972d
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrderbookMsg {
    version: Option<u64>,
    asks: Vec<[f64; 3]>,
    bids: Vec<[f64; 3]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    symbol: String,
    ts: i64,
    data: T,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawTradeMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, super::EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let raw_trade = ws_msg.data;

    let (quantity_base, quantity_quote, _) = calc_quantity_and_volume(
        super::EXCHANGE_NAME,
        market_type,
        &pair,
        raw_trade.p,
        raw_trade.v,
    );

    let trade = TradeMsg {
        exchange: super::EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.t,
        price: raw_trade.p,
        quantity_base,
        quantity_quote,
        quantity_contract: Some(raw_trade.v),
        side: if raw_trade.T == 2 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.t.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
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
    let symbol = ws_msg.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, super::EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;

    let msg_type = match ws_msg.channel.as_str() {
        "push.depth.full" => MessageType::L2TopK,
        "push.depth" => MessageType::L2Event,
        _ => panic!("Unsupported channel {}", ws_msg.channel),
    };

    let parse_order = |raw_order: &[f64; 3]| -> Order {
        let price = raw_order[0];
        let quantity = raw_order[1];
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(super::EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: super::EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type,
        timestamp: ws_msg.ts,
        seq_id: ws_msg.data.version,
        prev_seq_id: None,
        asks: ws_msg
            .data
            .asks
            .iter()
            .map(|x| parse_order(x))
            .collect::<Vec<Order>>(),
        bids: ws_msg
            .data
            .bids
            .iter()
            .map(|x| parse_order(x))
            .collect::<Vec<Order>>(),
        snapshot: msg_type == MessageType::L2TopK,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
