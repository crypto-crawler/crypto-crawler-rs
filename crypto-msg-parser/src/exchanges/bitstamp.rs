use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
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

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if let Some(channel) = json_obj.get("channel") {
        let symbol = channel.as_str().unwrap().split('_').last().unwrap();
        Ok(symbol.to_string())
    } else if json_obj.contains_key("asks") && json_obj.contains_key("bids") {
        // l2_snapshot has no symbol
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if let Some(data) = json_obj.get("data") {
        Ok(Some(
            data["microtimestamp"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap()
                / 1000,
        ))
    } else if let Some(microtimestamp) = json_obj.get("microtimestamp") {
        Ok(Some(
            microtimestamp.as_str().unwrap().parse::<i64>().unwrap() / 1000,
        ))
    } else {
        Err(SimpleError::new(format!(
            "No microtimestamp field in  {}",
            msg
        )))
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotTradeMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.channel.split('_').last().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
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

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SpotOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<SpotOrderbookMsg>",
            msg
        ))
    })?;
    let symbol = ws_msg.channel.split('_').last().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME)
        .ok_or_else(|| SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg)))?;
    let msg_type = if ws_msg.channel.starts_with("diff_order_book_") {
        MessageType::L2Event
    } else {
        MessageType::L2TopK
    };
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
        msg_type,
        timestamp: raw_orderbook.microtimestamp.parse::<i64>().unwrap() / 1000,
        seq_id: None,
        prev_seq_id: None,
        asks: raw_orderbook.asks.iter().map(|x| parse_order(x)).collect(),
        bids: raw_orderbook.bids.iter().map(|x| parse_order(x)).collect(),
        snapshot: ws_msg.channel.starts_with("order_book_"),
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
