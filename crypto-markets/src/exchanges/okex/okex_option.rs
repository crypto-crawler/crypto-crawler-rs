use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::Result,
    market::{Fees, Precision},
    Market,
};

use chrono::DateTime;
use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#option-option---instrument>
#[derive(Serialize, Deserialize)]
struct OptionMarket {
    instrument_id: String,
    underlying: String,
    settlement_currency: String,
    contract_val: String,
    option_type: String,
    strike: String,
    tick_size: String,
    lot_size: String,
    listing: String,
    delivery: String,
    state: String,
    trading_start_time: String,
    timestamp: String,
    category: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#option-option---instrument>
fn fetch_option_markets_raw() -> Result<Vec<OptionMarket>> {
    let txt = http_get("https://www.okex.com/api/option/v3/underlying", None)?;
    let underlying_indexes = serde_json::from_str::<Vec<String>>(&txt)?;

    let mut markets = Vec::<OptionMarket>::new();
    for index in underlying_indexes.iter() {
        let url = format!("https://www.okex.com/api/option/v3/instruments/{}", index);
        let txt = http_get(url.as_str(), None)?;
        let mut arr = serde_json::from_str::<Vec<OptionMarket>>(&txt)?;
        markets.append(&mut arr);
    }

    Ok(markets)
}

pub(super) fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_option_markets_raw()?
        .into_iter()
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &OptionMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.instrument_id, "okex").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let (base_id, quote_id) = {
        let v: Vec<&str> = raw_market.underlying.split('-').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let delivery_time = DateTime::parse_from_rfc3339(&raw_market.delivery)
        .unwrap()
        .timestamp_millis();
    Market {
        exchange: "okex".to_string(),
        market_type: MarketType::EuropeanOption,
        symbol: raw_market.instrument_id.to_string(),
        base_id,
        quote_id,
        base,
        quote,
        active: true,
        margin: true,
        // see https://www.okex.com/fees.html
        fees: Fees {
            maker: 0.0002,
            taker: 0.0005,
        },
        precision: Precision {
            tick_size: raw_market.tick_size.parse::<f64>().unwrap(),
            lot_size: raw_market.lot_size.parse::<f64>().unwrap(),
        },
        quantity_limit: None,
        contract_value: Some(raw_market.contract_val.parse::<f64>().unwrap()),
        delivery_date: Some(delivery_time as u64),
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_option_markets() -> Result<Vec<Market>> {
    let markets = fetch_option_markets_raw()?
        .into_iter()
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
