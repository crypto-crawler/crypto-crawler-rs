use std::time::Duration;

use crypto_market_type::MarketType;
use reqwest::{header, Result};
use serde_json::Value;

pub(super) fn http_get(url: &str) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::builder()
         .default_headers(headers)
         .timeout(Duration::from_secs(10))
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.get(url).send()?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text()?),
        Err(error) => Err(error),
    }
}

const PRECISION: f64 = 1000000000.0; // 9 decimals

pub fn round(f: f64) -> f64 {
    (f * PRECISION).round() / PRECISION
}

// returns (quantity_base, quantity_quote, quantity_contract)
pub(super) fn calc_quantity_and_volume(
    exchange: &str,
    market_type: MarketType,
    pair: &str,
    price: f64,
    quantity: f64,
) -> (f64, f64, Option<f64>) {
    let contract_value =
        crypto_contract_value::get_contract_value(exchange, market_type, pair).unwrap() as f64;
    match market_type {
        MarketType::Spot => (quantity, round(quantity * price), None),
        MarketType::InverseSwap | MarketType::InverseFuture => {
            let quantity_quote = quantity * contract_value;
            (quantity_quote / price, quantity_quote, Some(quantity))
        }
        MarketType::LinearSwap | MarketType::LinearFuture | MarketType::Move | MarketType::BVOL => {
            let quantity_base = quantity * contract_value;
            (
                round(quantity_base),
                round(quantity_base * price),
                Some(quantity),
            )
        }
        MarketType::EuropeanOption => {
            let quantity_base = quantity * contract_value;
            (quantity_base, quantity_base * price, Some(quantity))
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

const MAX_UNIX_TIMESTAMP: i64 = 10_i64.pow(10) - 1;
const MAX_UNIX_TIMESTAMP_MS: i64 = 10_i64.pow(13) - 1;

// Convert a UNIX timestamp to a timestamp in milliseconds if needed.
const fn convert_unix_timestamp_if_needed(ts: i64) -> i64 {
    if ts <= MAX_UNIX_TIMESTAMP {
        ts * 1000
    } else if ts <= MAX_UNIX_TIMESTAMP_MS {
        ts
    } else {
        ts / 1000
    }
}

// Convert a JSON value to a timestamp in milliseconds.
pub(super) fn convert_timestamp(v: &Value) -> Option<i64> {
    if v.is_i64() {
        let ts = v.as_i64().unwrap();
        Some(convert_unix_timestamp_if_needed(ts))
    } else if v.is_string() {
        let s = v.as_str().unwrap();
        let ts = s
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("{}", v.to_string()));
        Some(convert_unix_timestamp_if_needed(ts))
    } else {
        None
    }
}
