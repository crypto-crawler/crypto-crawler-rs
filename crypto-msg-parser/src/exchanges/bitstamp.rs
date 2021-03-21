use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bitstamp";

// see "Live ticker" at https://www.bitstamp.net/websocket/v2/
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    microtimestamp: String, // Trade microtimestamp
    amount: f64,            // Trade amount
    buy_order_id: i64,      // Trade buy order ID
    sell_order_id: i64,     // Trade sell order ID.
    amount_str: String,     // Trade amount represented in string format
    price_str: String,      // Trade price represented in string format
    timestamp: String,      // Trade timestamp
    price: f64,             // Trade price
    #[serde(rename = "type")]
    type_: i64, // Trade type (0 - buy; 1 - sell)
    id: i64,                // Trade unique ID
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    event: String,
    data: T,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg)?;
    let symbol = ws_msg.channel.strip_prefix("live_trades_").unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_trade = ws_msg.data;

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: raw_trade.microtimestamp.parse::<i64>().unwrap() / 1000,
        price: raw_trade.price,
        quantity: raw_trade.amount,
        volume: raw_trade.price * raw_trade.amount,
        side: if raw_trade.type_ == 1 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.id.to_string(),
        raw: serde_json::to_value(&raw_trade).unwrap(),
    };

    Ok(vec![trade])
}
