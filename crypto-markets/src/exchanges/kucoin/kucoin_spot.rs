use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, Precision, QuantityLimit,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    symbol: String,
    name: String,
    baseCurrency: String,
    quoteCurrency: String,
    feeCurrency: String,
    market: String,
    baseMinSize: String,
    quoteMinSize: String,
    baseMaxSize: String,
    quoteMaxSize: String,
    baseIncrement: String,
    quoteIncrement: String,
    priceIncrement: String,
    priceLimitRate: String,
    isMarginEnabled: bool,
    enableTrading: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: String,
    data: Vec<SpotMarket>,
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.kucoin.com/api/v1/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.code != "200000" {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .into_iter()
            .filter(|x| x.enableTrading)
            .collect::<Vec<SpotMarket>>();
        Ok(markets)
    }
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
            let pair = crypto_pair::normalize_pair(&m.symbol, "kucoin").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "kucoin".to_string(),
                market_type: MarketType::Spot,
                symbol: m.symbol,
                base_id: m.baseCurrency,
                quote_id: m.quoteCurrency,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: m.enableTrading,
                margin: m.isMarginEnabled,
                // see https://www.bitstamp.net/fee-schedule/
                fees: Fees {
                    maker: 0.005,
                    taker: 0.005,
                },
                precision: Precision {
                    tick_size: m.priceIncrement.parse::<f64>().unwrap(),
                    lot_size: m.baseIncrement.parse::<f64>().unwrap(),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.baseMinSize.parse::<f64>().ok(),
                    max: Some(m.baseMaxSize.parse::<f64>().unwrap()),
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
