mod dydx_swap;
mod message;

use crate::{OrderBookMsg, TradeMsg};

use crypto_market_type::MarketType;
use serde_json::Value;
use simple_error::SimpleError;

use self::message::{L2SnapshotRawMsg, WebsocketMsg};

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        Ok(ws_msg.id)
    } else if serde_json::from_str::<L2SnapshotRawMsg>(msg).is_ok() {
        Ok("NONE".to_string())
    } else if msg.starts_with(r#"{"markets":"#) {
        // https://api.dydx.exchange/v3/markets
        Ok("ALL".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::extract_timestamp(msg),
        _ => Err(SimpleError::new(format!(
            "Unknown dYdX market type {}",
            market_type
        ))),
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::parse_trade(market_type, msg),
        _ => Err(SimpleError::new(format!(
            "Unknown dYdX market type {}",
            market_type
        ))),
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::parse_l2(market_type, msg, timestamp),
        _ => Err(SimpleError::new(format!(
            "Unknown dYdX market type {}",
            market_type
        ))),
    }
}
