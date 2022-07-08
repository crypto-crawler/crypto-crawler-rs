use super::utils::mexc_http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    displayName: String,
    displayNameEn: String,
    positionOpenType: i64,
    baseCoin: String,
    quoteCoin: String,
    settleCoin: String,
    contractSize: f64,
    minLeverage: i64,
    maxLeverage: i64,
    priceScale: i64,
    volScale: i64,
    amountScale: i64,
    priceUnit: f64,
    volUnit: i64,
    minVol: i64,
    maxVol: i64,
    bidLimitPriceRate: f64,
    askLimitPriceRate: f64,
    takerFeeRate: f64,
    makerFeeRate: f64,
    maintenanceMarginRate: f64,
    initialMarginRate: f64,
    state: i64,
    isNew: bool,
    isHot: bool,
    isHidden: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    code: i64,
    data: Vec<SwapMarket>,
}

// see <https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md#contract-interface-public>
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = mexc_http_get("https://contract.mexc.com/api/v1/contract/detail")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    Ok(resp
        .data
        .into_iter()
        .filter(|m| m.state == 0 && !m.isHidden)
        .collect())
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.settleCoin == m.quoteCoin)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.settleCoin == m.baseCoin)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &SwapMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.symbol, super::EXCHANGE_NAME).unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let market_type = if raw_market.settleCoin == raw_market.quoteCoin {
        MarketType::LinearSwap
    } else if raw_market.settleCoin == raw_market.baseCoin {
        MarketType::InverseSwap
    } else {
        panic!("unexpected market type");
    };

    Market {
        exchange: super::EXCHANGE_NAME.to_string(),
        market_type,
        symbol: raw_market.symbol.to_string(),
        base_id: raw_market.baseCoin.to_string(),
        quote_id: raw_market.quoteCoin.to_string(),
        settle_id: Some(raw_market.settleCoin.to_string()),
        base,
        quote,
        settle: Some(raw_market.settleCoin.to_string()),
        active: raw_market.state == 0 && !raw_market.isHidden,
        margin: true,
        fees: Fees {
            maker: raw_market.makerFeeRate,
            taker: raw_market.takerFeeRate,
        },
        precision: Precision {
            tick_size: raw_market.priceUnit,
            lot_size: raw_market.volUnit as f64,
        },
        quantity_limit: Some(QuantityLimit {
            min: Some(raw_market.minVol as f64),
            max: Some(raw_market.maxVol as f64),
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(raw_market.contractSize),
        delivery_date: None,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.settleCoin == m.quoteCoin)
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.settleCoin == m.baseCoin)
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
