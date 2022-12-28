use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, MarketType, Precision,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct CoinConfig {
    makerFeeRate: serde_json::Value, // String or i64
    minTxAmt: Option<String>,
    name: String,
    depositStatus: String,
    fullName: String,
    takerFeeRate: serde_json::Value, // String or i64
    minWithdraw: String,
    withdrawFee: String,
    withdrawStatus: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PercentPrice {
    multiplierDown: String,
    multiplierUp: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotConfig {
    symbol: String,
    percentPrice: PercentPrice,
    accuracy: Vec<String>,
    openPrice: String,
    openTime: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Data {
    coinConfig: Vec<CoinConfig>,
    spotConfig: Vec<SpotConfig>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    data: Data,
    code: String,
    msg: String,
    timestamp: i64,
}

// see https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/rest-api.md#2-config-detail
fn fetch_spot_coing() -> Result<Data> {
    let txt = http_get("https://global-openapi.bithumb.pro/openapi/v1/spot/config", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.code != "0" { Err(Error(txt)) } else { Ok(resp.data) }
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols =
        fetch_spot_coing()?.spotConfig.into_iter().map(|m| m.symbol).collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_coing()?
        .spotConfig
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m).unwrap().as_object().unwrap().clone();
            let pair = crypto_pair::normalize_pair(&m.symbol, "bithumb").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let (base_id, quote_id) = {
                let v: Vec<&str> = m.symbol.split('-').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "bithumb".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id,
                quote_id,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: true,
                margin: false,
                // see https://www.bitglobal.com/en-us/fee
                fees: Fees { maker: 0.001, taker: 0.001 },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.accuracy[0].parse::<u32>().unwrap()) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.accuracy[1].parse::<u32>().unwrap()) as f64),
                },
                quantity_limit: None,
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
