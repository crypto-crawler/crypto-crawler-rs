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

// see <https://www.okex.com/docs/en/#futures-contract_information>
#[derive(Serialize, Deserialize)]
struct FutureMarket {
    instrument_id: String,
    underlying: String,
    base_currency: String,
    quote_currency: String,
    settlement_currency: String,
    contract_val: String,
    listing: String,
    delivery: String,
    tick_size: String,
    trade_increment: String,
    alias: String,
    is_inverse: String,
    contract_val_currency: String,
    category: String,
    underlying_index: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#futures-contract_information>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = http_get("https://www.okex.com/api/futures/v3/instruments", None)?;
    let markets = serde_json::from_str::<Vec<FutureMarket>>(&txt)?;
    Ok(markets)
}

pub(super) fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "false")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "true")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &FutureMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.instrument_id, "okex").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let delivery_time =
        DateTime::parse_from_rfc3339(format!("{}T00:00:00+00:00", raw_market.delivery).as_str())
            .unwrap()
            .timestamp_millis();
    Market {
        exchange: "okex".to_string(),
        market_type: if raw_market.is_inverse == "true" {
            MarketType::InverseFuture
        } else {
            MarketType::LinearFuture
        },
        symbol: raw_market.instrument_id.to_string(),
        base_id: raw_market.base_currency.to_string(),
        quote_id: raw_market.quote_currency.to_string(),
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
            lot_size: raw_market.trade_increment.parse::<f64>().unwrap(),
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

pub(super) fn fetch_linear_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "false")
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "true")
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
