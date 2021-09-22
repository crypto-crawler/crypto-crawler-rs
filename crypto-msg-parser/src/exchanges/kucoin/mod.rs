mod kucoin_spot;
mod kucoin_swap;
mod message;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::{Result, Value};

use self::message::WebsocketMsg;

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let pos = ws_msg.topic.rfind(':').unwrap();
    let symbol = &ws_msg.topic[pos + 1..];
    Some(symbol.to_string())
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Spot {
        kucoin_spot::parse_trade(msg)
    } else {
        kucoin_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    if market_type == MarketType::Spot {
        kucoin_spot::parse_l2(msg)
    } else {
        kucoin_swap::parse_l2(market_type, msg)
    }
}
