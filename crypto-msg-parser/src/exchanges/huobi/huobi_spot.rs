use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use super::message::WebsocketMsg;

const EXCHANGE_NAME: &str = "huobi";

// see https://huobiapi.github.io/docs/spot/v1/en/#trade-detail
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotTradeMsg {
    ts: i64,
    tradeId: i64,
    amount: f64,
    price: f64,
    direction: String, // sell, buy
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://huobiapi.github.io/docs/spot/v1/en/#market-by-price-incremental-update
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotOrderbookMsg {
    seqNum: i64,
    prevSeqNum: i64,
    asks: Option<Vec<[f64; 2]>>,
    bids: Option<Vec<[f64; 2]>>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct TradeTick {
    id: i64,
    ts: i64,
    data: Vec<SpotTradeMsg>,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<TradeTick>>(msg)?;

    let symbol = {
        let v: Vec<&str> = ws_msg.ch.split('.').collect();
        v[1]
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let mut trades: Vec<TradeMsg> = ws_msg
        .tick
        .data
        .into_iter()
        .map(|raw_trade| TradeMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol: symbol.to_string(),
            pair: pair.to_string(),
            msg_type: MessageType::Trade,
            timestamp: raw_trade.ts,
            price: raw_trade.price,
            quantity_base: raw_trade.amount,
            quantity_quote: raw_trade.price * raw_trade.amount,
            quantity_contract: None,
            side: if raw_trade.direction == "sell" {
                TradeSide::Sell
            } else {
                TradeSide::Buy
            },
            trade_id: raw_trade.tradeId.to_string(),
            json: serde_json::to_string(&raw_trade).unwrap(),
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg)?;
    let symbol = {
        let v: Vec<&str> = ws_msg.ch.split('.').collect();
        v[1]
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let timestamp = ws_msg.ts;

    let parse_order = |raw_order: &[f64; 2]| -> Order {
        let price = raw_order[0];
        let quantity_base = raw_order[1];

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
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp,
        seq_first: Some(ws_msg.tick.seqNum as u64),
        seq_last: None,
        asks: ws_msg
            .tick
            .asks
            .into_iter()
            .flatten()
            .map(|x| parse_order(&x))
            .collect(),
        bids: ws_msg
            .tick
            .bids
            .into_iter()
            .flatten()
            .map(|x| parse_order(&x))
            .collect(),
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
