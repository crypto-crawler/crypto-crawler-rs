use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "https://api.dydx.exchange";

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PerpetualMarket {
    market: String,
    status: String,
    baseAsset: String,
    quoteAsset: String,
    stepSize: String,
    tickSize: String,
    minOrderSize: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MarketsResponse {
    markets: HashMap<String, PerpetualMarket>,
}

// See https://docs.dydx.exchange/#get-markets
fn fetch_markets_raw() -> Result<Vec<PerpetualMarket>> {
    let txt = http_get(format!("{}/v3/markets", BASE_URL).as_str(), None)?;
    let resp = serde_json::from_str::<MarketsResponse>(&txt)?;
    Ok(resp
        .markets
        .values()
        .cloned()
        .filter(|x| x.status == "ONLINE")
        .collect::<Vec<PerpetualMarket>>())
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols = markets
        .into_iter()
        .map(|m| m.market)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.market, "dydx").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "dydx".to_string(),
                market_type: MarketType::LinearSwap,
                symbol: m.market,
                base_id: m.baseAsset,
                quote_id: m.quoteAsset,
                settle_id: Some(quote.clone()),
                base,
                quote: quote.clone(),
                settle: Some(quote),
                active: m.status == "ONLINE",
                margin: true,
                // see https://trade.dydx.exchange/portfolio/fees
                fees: Fees {
                    maker: 0.0005,
                    taker: 0.0001,
                },
                precision: Precision {
                    tick_size: m.tickSize.parse::<f64>().unwrap(),
                    lot_size: m.stepSize.parse::<f64>().unwrap(),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.minOrderSize.parse::<f64>().ok(),
                    max: None,
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: Some(1.0),
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
