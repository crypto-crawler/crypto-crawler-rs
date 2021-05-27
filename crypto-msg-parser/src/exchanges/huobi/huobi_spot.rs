use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize)]
struct TradeTick {
    id: i64,
    ts: i64,
    data: Vec<SpotTradeMsg>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    ch: String,
    ts: i64,
    tick: T,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<TradeTick>>(msg)?;

    let symbol = {
        let v: Vec<&str> = ws_msg.ch.split('.').collect();
        v[1]
    };
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let trades: Vec<TradeMsg> = ws_msg
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
            raw: serde_json::to_value(&raw_trade).unwrap(),
        })
        .collect();

    Ok(trades)
}
