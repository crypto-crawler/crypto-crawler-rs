use crypto_market_type::MarketType;

use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

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

// see "Live full order book" at https://www.bitstamp.net/websocket/v2/
#[derive(Serialize, Deserialize)]
struct SpotOrderbookMsg {
    timestamp: String,      // Trade timestamp
    microtimestamp: String, // Trade microtimestamp
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    event: String,
    data: T,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let channel = ws_msg.channel;
    let pos = channel.rfind('_').unwrap();
    let symbol = &channel[pos + 1..];
    Some(symbol.to_string())
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
        quantity_base: raw_trade.amount,
        quantity_quote: raw_trade.price * raw_trade.amount,
        quantity_contract: None,
        side: if raw_trade.type_ == 1 {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.id.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg)?;
    let symbol = ws_msg.channel.strip_prefix("diff_order_book_").unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    let raw_orderbook = ws_msg.data;

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
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: raw_orderbook.microtimestamp.parse::<i64>().unwrap() / 1000,
        asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
        bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}
