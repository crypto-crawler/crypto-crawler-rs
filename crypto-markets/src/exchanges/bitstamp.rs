use super::utils::http_get;
use crate::{error::Result, Fees, Market, MarketType, Precision};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => fetch_spot_markets(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    base_decimals: i64,
    minimum_order: String,
    name: String,
    counter_decimals: i64,
    trading: String,
    url_symbol: String,
    description: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.bitstamp.net/api/>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.bitstamp.net/api/v2/trading-pairs-info/", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets.into_iter().filter(|m| m.trading == "Enabled").collect())
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols =
        fetch_spot_markets_raw()?.into_iter().map(|m| m.url_symbol).collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m).unwrap().as_object().unwrap().clone();
            let pair = crypto_pair::normalize_pair(&m.url_symbol, "bitstamp").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let (base_id, quote_id) = {
                let v: Vec<&str> = m.name.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "bitstamp".to_string(),
                market_type: MarketType::Spot,
                symbol: m.url_symbol,
                base_id,
                quote_id,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: true,
                margin: true,
                // see https://www.bitstamp.net/fee-schedule/
                fees: Fees { maker: 0.005, taker: 0.005 },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.base_decimals as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.counter_decimals as u32) as f64),
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
