mod binance_all;
mod binance_option;

use std::collections::HashMap;

use crypto_market_type::MarketType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::{Result, Value};

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    let data = obj.get("data").unwrap();
    let symbol = data["s"].as_str().unwrap();
    Some(symbol.to_string())
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::EuropeanOption {
        binance_option::parse_trade(msg)
    } else {
        binance_all::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        binance_all::parse_funding_rate(market_type, msg)
    } else {
        panic!("Binance {} does NOT have funding rates", market_type);
    }
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    if market_type == MarketType::EuropeanOption {
        Ok(Vec::new())
    } else {
        binance_all::parse_l2(market_type, msg)
    }
}
