use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "mxc";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    p: String, // price
    q: String, // quantity
    T: i64,    // 1, buy; 2, sell
    t: i64,    // timestamp
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Order {
    p: String,
    q: String,
    a: String,
}

#[derive(Serialize, Deserialize)]
struct PushSymbolData {
    deals: Option<Vec<RawTradeMsg>>,
    asks: Option<Vec<Order>>,
    bids: Option<Vec<Order>>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    symbol: String,
    data: T,
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let arr = serde_json::from_str::<Vec<Value>>(msg)?;
    assert_eq!(arr.len(), 2);
    let ws_msg: WebsocketMsg<PushSymbolData> = serde_json::from_value(arr[1].clone())?;
    if ws_msg.data.deals.is_none() {
        return Ok(Vec::new());
    }

    let raw_trades = ws_msg.data.deals.unwrap();
    let symbol = ws_msg.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            let price = raw_trade.p.parse::<f64>().unwrap();
            let quantity = raw_trade.q.parse::<f64>().unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.t,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side: if raw_trade.T == 2 {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.t.to_string(),
                raw: serde_json::to_value(&raw_trade).unwrap(),
            }
        })
        .collect();

    Ok(trades)
}
