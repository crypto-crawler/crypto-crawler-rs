use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::Result,
    market::{Fees, Precision, QuantityLimit},
    Market,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#spot-currency>
#[derive(Serialize, Deserialize)]
struct SpotMarket {
    instrument_id: String,
    base_currency: String,
    quote_currency: String,
    min_size: String,
    size_increment: String,
    tick_size: String,
    category: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#spot-currency>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.okex.com/api/spot/v3/instruments", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets)
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &SpotMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.instrument_id, "okex").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    Market {
        exchange: "okex".to_string(),
        market_type: MarketType::Spot,
        symbol: raw_market.instrument_id.to_string(),
        base_id: raw_market.base_currency.to_string(),
        quote_id: raw_market.quote_currency.to_string(),
        base,
        quote,
        active: true,
        margin: true,
        // see https://www.okex.com/fees.html
        fees: Fees {
            maker: 0.0008,
            taker: 0.001,
        },
        precision: Precision {
            tick_size: raw_market.tick_size.parse::<f64>().unwrap(),
            lot_size: raw_market.size_increment.parse::<f64>().unwrap(),
        },
        quantity_limit: Some(QuantityLimit {
            min: raw_market.min_size.parse::<f64>().unwrap(),
            max: None,
        }),
        contract_value: None,
        delivery_date: None,
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
