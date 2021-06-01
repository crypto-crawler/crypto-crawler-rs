use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
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
    asks: Vec<[String; 3]>,
    bids: Vec<[String; 3]>,
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

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SpotTradeMsg>>>(msg)?;
    let symbol = ws_msg.params.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let trades: Vec<TradeMsg> = ws_msg
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
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg)?;
    debug_assert_eq!(ws_msg.action, "Pushdata.depth");
    let symbol = ws_msg.params.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let parse_order = |raw_order: &[String; 3]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity_base = raw_order[1].parse::<f64>().unwrap();
        let quantity_quote = raw_order[2].parse::<f64>().unwrap();

        vec![price, quantity_base, quantity_quote]
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair: pair.to_string(),
        msg_type: MessageType::L2Event,
        timestamp: ws_msg.time,
        asks: ws_msg.data.asks.iter().map(|x| parse_order(x)).collect(),
        bids: ws_msg.data.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot: false,
        raw: serde_json::from_str(msg)?,
    };

    Ok(vec![orderbook])
}
