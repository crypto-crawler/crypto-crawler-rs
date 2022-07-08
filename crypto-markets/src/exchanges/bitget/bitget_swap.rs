use std::collections::HashMap;

use super::{super::utils::http_get, EXCHANGE_NAME};
use crate::{
    error::{Error, Result},
    Fees, Market, Precision, QuantityLimit,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// See https://bitgetlimited.github.io/apidoc/en/mix/#get-all-symbols
#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,                  // symbol Id
    baseCoin: String,                // Base coin
    quoteCoin: String,               // Denomination coin
    buyLimitPriceRatio: String,      // Buy price limit ratio 1%
    sellLimitPriceRatio: String,     // Sell price limit ratio 1%
    feeRateUpRatio: String,          // Rate of increase in handling fee%
    takerFeeRate: String,            // Taker fee rate%
    makerFeeRate: String,            // Market fee rate%
    openCostUpRatio: String,         // Percentage of increase in opening cost%
    supportMarginCoins: Vec<String>, // Support margin currency
    minTradeNum: String,             // Minimum number of openings
    priceEndStep: String,            // Price step
    pricePlace: String,              // Price decimal places
    volumePlace: String,             // Number of decimal places
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    code: String,
    msg: String,
    data: Vec<SwapMarket>,
    requestTime: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://bitgetlimited.github.io/apidoc/en/mix/#get-all-symbols
// product_type: umcbl, LinearSwap; dmcbl, InverseSwap;
fn fetch_swap_markets_raw(product_type: &str) -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        format!(
            "https://api.bitget.com/api/mix/v1/market/contracts?productType={}",
            product_type
        )
        .as_str(),
        None,
    )?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.msg != "success" {
        Err(Error(txt))
    } else {
        Ok(resp.data)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw("dmcbl")?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw("umcbl")?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw("dmcbl")?
        .into_iter()
        .map(to_market)
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw("umcbl")?
        .into_iter()
        .map(to_market)
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn to_market(m: SwapMarket) -> Market {
    let market_type = if m.symbol.ends_with("_UMCBL") {
        MarketType::LinearSwap
    } else if m.symbol.ends_with("_DMCBL") {
        MarketType::InverseSwap
    } else {
        panic!("unexpected symbol: {}", m.symbol);
    };
    Market {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: m.symbol.clone(),
        base_id: m.baseCoin.clone(),
        quote_id: m.quoteCoin.clone(),
        settle_id: Some(m.supportMarginCoins[0].clone()),
        base: m.baseCoin.clone(),
        quote: m.quoteCoin.clone(),
        settle: Some(m.supportMarginCoins[0].clone()),
        active: true,
        margin: true,
        fees: Fees {
            maker: m.makerFeeRate.parse::<f64>().unwrap(),
            taker: m.takerFeeRate.parse::<f64>().unwrap(),
        },
        precision: Precision {
            tick_size: 1.0 / (10_i64.pow(m.pricePlace.parse::<u32>().unwrap()) as f64),
            lot_size: 1.0 / (10_i64.pow(m.volumePlace.parse::<u32>().unwrap()) as f64),
        },
        quantity_limit: Some(QuantityLimit {
            min: m.minTradeNum.parse::<f64>().ok(),
            max: None,
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(1.0), // TODO:
        delivery_date: None,
        info: serde_json::to_value(&m)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}
