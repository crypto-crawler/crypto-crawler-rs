use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    #[serde(default)]
    symbol: String,
    amountScale: u32,
    minAmount: f64,
    minSize: f64,
    priceScale: u32,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.zb.com/data/v1/markets", None)?;
    let m = serde_json::from_str::<HashMap<String, SpotMarket>>(&txt)?;
    let mut markets = Vec::new();
    for (symbol, mut market) in m {
        market.symbol = symbol;
        markets.push(market);
    }
    Ok(markets)
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets: Vec<Market> = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let (base_id, quote_id) = {
                let v: Vec<&str> = m.symbol.split('_').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let pair = crypto_pair::normalize_pair(&m.symbol, "zb").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "zb".to_string(),
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
                // see https://www.zb.com/help/rate/6
                fees: Fees {
                    maker: 0.002,
                    taker: 0.002,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.priceScale) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.amountScale) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: Some(m.minAmount),
                    max: None,
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect();
    Ok(markets)
}
