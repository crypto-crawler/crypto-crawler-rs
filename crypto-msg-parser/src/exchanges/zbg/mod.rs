mod zbg_spot;
mod zbg_swap;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Spot {
        zbg_spot::parse_trade(msg)
    } else {
        zbg_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    if market_type == MarketType::Spot {
        zbg_spot::parse_l2(msg)
    } else {
        zbg_swap::parse_l2(market_type, msg)
    }
}
