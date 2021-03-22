mod huobi_inverse;
mod huobi_linear;
mod huobi_spot;

use crypto_market_type::MarketType;

use crate::TradeMsg;

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_trade(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_trade(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap => {
            huobi_linear::parse_trade(market_type, msg)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}
