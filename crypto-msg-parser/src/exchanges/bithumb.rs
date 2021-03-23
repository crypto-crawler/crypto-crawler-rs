use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
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

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)?;
    let raw_trades = if ws_msg.code == "00006" {
        // snapshot
        let ws_msg = serde_json::from_str::<WebsocketMsg<Vec<SpotTradeMsg>>>(msg)?;
        ws_msg.data
    } else if ws_msg.code == "00007" {
        // updates
        let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg)?;
        vec![ws_msg.data]
    } else {
        panic!("Invalid trade msg {}", msg);
    };
    let trades: Vec<TradeMsg> = raw_trades
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
                quantity,
                volume: price * quantity,
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
