mod binance;
mod binance_option;

use crypto_market_type::MarketType;

use crate::TradeMsg;

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Option {
        binance_option::parse_trade(msg)
    } else {
        binance::parse_trade(market_type, msg)
    }
}
