mod dydx_swap;
mod message;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::{Result, Value};

use self::message::WebsocketMsg;

pub(crate) fn extract_symbol(_market_type_: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    Some(ws_msg.id)
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::parse_trade(market_type, msg),
        _ => panic!("Unknown market type {}", market_type),
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::parse_l2(market_type, msg, timestamp),
        _ => panic!("Unknown market type {}", market_type),
    }
}
