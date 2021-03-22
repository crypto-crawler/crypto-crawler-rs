mod gate_spot;
mod gate_swap;

use crypto_market_type::MarketType;

use crate::TradeMsg;

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Spot {
        gate_spot::parse_trade(msg)
    } else {
        gate_swap::parse_trade(market_type, msg)
    }
}
