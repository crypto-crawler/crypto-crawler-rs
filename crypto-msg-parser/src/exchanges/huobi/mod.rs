mod funding_rate;
mod huobi_inverse;
mod huobi_linear;
mod huobi_spot;
mod message;

use crypto_market_type::MarketType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::{Result, Value};

use message::WebsocketMsg;

pub(crate) fn extract_symbol(_market_type_: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let v = ws_msg.ch.split('.').collect::<Vec<&str>>();
    Some(v[1].to_string())
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_trade(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_trade(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_linear::parse_trade(market_type, msg)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        funding_rate::parse_funding_rate(market_type, msg)
    } else {
        panic!("Huobi {} does NOT have funding rates", market_type);
    }
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_l2(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_l2(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_inverse::parse_l2(market_type, msg)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}
