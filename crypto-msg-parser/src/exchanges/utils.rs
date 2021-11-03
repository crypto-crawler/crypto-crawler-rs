use std::time::Duration;

use crypto_market_type::MarketType;
use reqwest::{header, Result};

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
        MarketType::Spot => (quantity, quantity * price, None),
        MarketType::InverseSwap | MarketType::InverseFuture | MarketType::EuropeanOption => {
            let quantity_quote = quantity * contract_value;
            (quantity_quote / price, quantity_quote, Some(quantity))
        }
        MarketType::LinearSwap | MarketType::LinearFuture | MarketType::Move | MarketType::BVOL => {
            if exchange == "bitmex" {
                let quantity_quote = quantity * contract_value;
                (quantity_quote / price, quantity_quote, Some(quantity))
            } else {
                let quantity_base = quantity * contract_value;
                (quantity_base, quantity_base * price, Some(quantity))
            }
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}
