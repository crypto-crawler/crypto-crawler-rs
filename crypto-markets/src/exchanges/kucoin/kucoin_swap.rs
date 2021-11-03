use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, Precision,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    rootSymbol: String,
    #[serde(rename = "type")]
    type_: String,
    expireDate: Option<u64>,
    baseCurrency: String,
    quoteCurrency: String,
    settleCurrency: String,
    maxOrderQty: i64,
    maxPrice: f64,
    lotSize: f64,
    tickSize: f64,
    indexPriceTickSize: f64,
    multiplier: f64,
    initialMargin: f64,
    maintainMargin: f64,
    maxRiskLimit: i64,
    minRiskLimit: i64,
    riskStep: i64,
    makerFeeRate: f64,
    takerFeeRate: f64,
    takerFixFee: f64,
    makerFixFee: f64,
    isDeleverage: bool,
    isQuanto: bool,
    isInverse: bool,
    markMethod: String,
    fairMethod: Option<String>,
    status: String,
    fundingFeeRate: Option<f64>,
    predictedFundingFeeRate: Option<f64>,
    openInterest: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: String,
    data: Vec<SwapMarket>,
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        "https://api-futures.kucoin.com/api/v1/contracts/active",
        None,
    )?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.code != "200000" {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .into_iter()
            .filter(|x| x.status == "Open")
            .collect::<Vec<SwapMarket>>();
        Ok(markets)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFWCSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| !x.isInverse && x.type_ == "FFWCSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFICSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

fn to_market(raw_market: &SwapMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.symbol, "kucoin").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let market_type = if raw_market.isInverse && raw_market.type_ == "FFWCSX" {
        MarketType::InverseSwap
    } else if !raw_market.isInverse && raw_market.type_ == "FFWCSX" {
        MarketType::LinearSwap
    } else if raw_market.isInverse && raw_market.type_ == "FFICSX" {
        MarketType::InverseFuture
    } else {
        panic!(
            "Failed to detect market_type {}",
            serde_json::to_string_pretty(raw_market).unwrap()
        );
    };

    Market {
        exchange: "kucoin".to_string(),
        market_type,
        symbol: raw_market.symbol.to_string(),
        base_id: raw_market.baseCurrency.to_string(),
        quote_id: raw_market.quoteCurrency.to_string(),
        base,
        quote,
        active: raw_market.status == "Open",
        margin: true,
        fees: Fees {
            maker: raw_market.makerFeeRate,
            taker: raw_market.takerFeeRate,
        },
        precision: Precision {
            tick_size: raw_market.tickSize,
            lot_size: raw_market.lotSize,
        },
        quantity_limit: None,
        contract_value: Some(raw_market.multiplier.abs()),
        delivery_date: raw_market.expireDate,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFWCSX")
        .map(|m| to_market(&m))
        .collect();
    Ok(markets)
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|x| !x.isInverse && x.type_ == "FFWCSX")
        .map(|m| to_market(&m))
        .collect();
    Ok(markets)
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFICSX")
        .map(|m| to_market(&m))
        .collect();
    Ok(markets)
}
