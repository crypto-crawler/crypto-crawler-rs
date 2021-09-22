use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "binance";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionTradeMsg {
    t: String, // Trade ID
    p: String, // Price
    q: String, // Quantity
    b: String, // Bid order ID
    a: String, // Ask order ID
    T: i64,    // Trade time
    s: String, // Side
    S: String, // Symbol
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionTradeAllMsg {
    e: String, // Event type
    E: i64,    // Event time
    s: String, // Symbol
    t: Vec<OptionTradeMsg>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    stream: String,
    data: T,
}

pub(crate) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)?;
    let data = obj.get("data").unwrap();
    let event_type = data.get("e").unwrap().as_str().unwrap();

    assert_eq!(event_type, "trade_all");

    let all_trades: OptionTradeAllMsg = serde_json::from_value(data.clone()).unwrap();
    let trades: Vec<TradeMsg> = all_trades
        .t
        .into_iter()
        .map(|trade| {
            let pair = crypto_pair::normalize_pair(&trade.S, EXCHANGE_NAME).unwrap();
            let price = trade.p.parse::<f64>().unwrap();
            let quantity = trade.q.parse::<f64>().unwrap();
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::EuropeanOption,
                symbol: trade.S.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp: trade.T,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: Some(quantity),
                side: if trade.s == "1" {
                    // TODO: find out the meaning of the field s
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: trade.a.to_string(),
                json: serde_json::to_string(&trade).unwrap(),
            }
        })
        .collect();
    Ok(trades)
}
