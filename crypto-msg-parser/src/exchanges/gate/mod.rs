mod gate_spot;
mod gate_spot_20210916;
mod gate_spot_current;
mod gate_swap;
mod messages;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Result;

pub(crate) fn extract_symbol(market_type: MarketType, msg: &str) -> Option<String> {
    if market_type == MarketType::Spot {
        gate_spot::extract_symbol(msg)
    } else {
        gate_swap::extract_symbol(market_type, msg)
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Spot {
        gate_spot::parse_trade(msg)
    } else {
        gate_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>> {
    if market_type == MarketType::Spot {
        gate_spot::parse_l2(
            msg,
            timestamp.expect("Gate spot orderbook messages don't have timestamp"),
        )
    } else {
        gate_swap::parse_l2(market_type, msg)
    }
}
