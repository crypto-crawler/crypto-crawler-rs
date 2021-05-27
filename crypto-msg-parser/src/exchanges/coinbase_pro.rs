use crypto_market_type::MarketType;

use crate::{MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "coinbase_pro";

// see https://docs.pro.coinbase.com/#match
#[derive(Serialize, Deserialize)]
struct SpotTradeMsg {
    #[serde(rename = "type")]
    type_: String,
    trade_id: i64,
    sequence: i64,
    maker_order_id: String,
    taker_order_id: String,
    time: String,
    product_id: String,
    size: String,
    price: String,
    side: String, // buy, sell
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let raw_trade = serde_json::from_str::<SpotTradeMsg>(msg)?;
    let timestamp = DateTime::parse_from_rfc3339(&raw_trade.time).unwrap();
    let price = raw_trade.price.parse::<f64>().unwrap();
    let quantity = raw_trade.size.parse::<f64>().unwrap();

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_trade.product_id.clone(),
        pair: crypto_pair::normalize_pair(&raw_trade.product_id, EXCHANGE_NAME).unwrap(),
        msg_type: MessageType::Trade,
        timestamp: timestamp.timestamp_millis(),
        price,
        quantity_base: quantity,
        quantity_quote: price * quantity,
        quantity_contract: None,
        side: if raw_trade.side == "sell" {
            TradeSide::Sell
        } else {
            TradeSide::Buy
        },
        trade_id: raw_trade.trade_id.to_string(),
        raw: serde_json::to_value(&raw_trade).unwrap(),
    };

    Ok(vec![trade])
}

pub(crate) fn parse_l2(_market_type: MarketType, _msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
