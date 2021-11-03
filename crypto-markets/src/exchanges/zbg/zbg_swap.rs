use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, Precision,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    currencyName: String,
    lotSize: String,
    contractId: i64,
    takerFeeRatio: String,
    commodityId: i64,
    currencyId: i64,
    contractUnit: String,
    makerFeeRatio: String,
    priceTick: String,
    commodityName: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResMsg {
    message: String,
    method: Option<String>,
    code: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    datas: Vec<SwapMarket>,
    resMsg: ResMsg,
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        "https://www.zbg.com/exchange/api/v1/future/common/contracts",
        None,
    )?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.resMsg.code != "1" {
        Err(Error(txt))
    } else {
        Ok(resp.datas)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .filter(|x| x.ends_with("_USD-R"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .filter(|x| x.ends_with("_USDT"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &SwapMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.symbol, "zbg").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let (base_id, quote_id) = {
        let v: Vec<&str> = raw_market.symbol.split('_').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let market_type = if raw_market.symbol.ends_with("_USD-R") {
        MarketType::InverseSwap
    } else if raw_market.symbol.ends_with("_USDT") {
        MarketType::LinearSwap
    } else {
        panic!(
            "Failed to detect market_type {}",
            serde_json::to_string_pretty(raw_market).unwrap()
        );
    };

    Market {
        exchange: "zbg".to_string(),
        market_type,
        symbol: raw_market.symbol.to_string(),
        base_id,
        quote_id,
        base,
        quote,
        active: true,
        margin: true,
        fees: Fees {
            maker: raw_market.makerFeeRatio.parse::<f64>().unwrap(),
            taker: raw_market.takerFeeRatio.parse::<f64>().unwrap(),
        },
        precision: Precision {
            tick_size: raw_market.priceTick.parse::<f64>().unwrap(),
            lot_size: raw_market.lotSize.parse::<f64>().unwrap(),
        },
        quantity_limit: None,
        contract_value: Some(raw_market.contractUnit.parse::<f64>().unwrap()),
        delivery_date: None,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.symbol.ends_with("_USD-R"))
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.symbol.ends_with("_USDT"))
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
