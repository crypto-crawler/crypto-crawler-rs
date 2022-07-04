use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, Precision, QuantityLimit,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    marginCurrencyName: String,
    buyerCurrencyName: String,
    sellerCurrencyName: String,
    canTrade: bool,
    canOpenPosition: bool,
    amountDecimal: u32,
    priceDecimal: u32,
    minAmount: String,
    maxAmount: String,
    status: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    code: i64,
    data: Vec<SwapMarket>,
}

// See https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#71-trading-pair
fn fetch_swap_markets_internal(url: &str) -> Result<Vec<SwapMarket>> {
    let txt = http_get(url, None)?;
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    if resp.code == 10000 {
        Ok(resp
            .data
            .into_iter()
            .filter(|m| m.canTrade && m.canOpenPosition && m.status == 1)
            .collect())
    } else {
        Err(Error(txt))
    }
}

fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let usdt_markets =
        fetch_swap_markets_internal("https://fapi.zb.com/Server/api/v2/config/marketList")?;
    let qc_markets =
        fetch_swap_markets_internal("https://fapi.zb.com/qc/Server/api/v2/config/marketList")?;
    Ok(usdt_markets
        .into_iter()
        .chain(qc_markets.into_iter())
        .collect())
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &SwapMarket) -> Market {
    let (base_id, quote_id) = {
        let v: Vec<&str> = raw_market.symbol.split('_').collect();
        (v[0].to_string(), v[1].to_string())
    };

    let pair = crypto_pair::normalize_pair(&raw_market.symbol, "zb").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };

    Market {
        exchange: "zb".to_string(),
        market_type: MarketType::LinearSwap,
        symbol: raw_market.symbol.to_string(),
        base_id,
        quote_id,
        settle_id: Some(raw_market.marginCurrencyName.to_uppercase()),
        base,
        quote,
        settle: Some(raw_market.marginCurrencyName.to_uppercase()),
        active: true,
        margin: true,
        // see https://www.zb.com/help/rate/20
        fees: Fees {
            maker: 0.0005,
            taker: 0.00075,
        },
        precision: Precision {
            tick_size: 1.0 / (10_i64.pow(raw_market.priceDecimal) as f64),
            lot_size: 1.0 / (10_i64.pow(raw_market.amountDecimal) as f64),
        },
        quantity_limit: Some(QuantityLimit {
            min: raw_market.minAmount.parse::<f64>().ok(),
            max: Some(raw_market.maxAmount.parse::<f64>().unwrap()),
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(1.0),
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
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
