mod utils;

use crypto_pair::normalize_pair;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "ftx";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FtxMarket {
    name: String,
    baseCurrency: Option<String>,
    quoteCurrency: Option<String>,
    underlying: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    result: Vec<FtxMarket>,
}

fn fetch_markets_raw() -> Vec<FtxMarket> {
    let txt = http_get("https://ftx.com/api/markets").unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    assert!(resp.success);
    resp.result
        .into_iter()
        .filter(|market| market.name.contains('-') || market.name.contains('/'))
        .collect()
}

#[test]
fn verify_all_symbols() {
    let markets = fetch_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.name, EXCHANGE_NAME).unwrap();
        let pair_expected = if market.type_ == "spot" {
            // spot
            market.name.clone()
        } else if market.type_ == "future" {
            if market.name.ends_with("-PERP") {
                format!("{}/USD", market.underlying.clone().unwrap())
            } else if market.name.contains("-MOVE-") {
                format!("{}-MOVE/USD", market.underlying.clone().unwrap())
            } else if market.name.contains("BVOL/") {
                format!(
                    "{}/{}",
                    market.baseCurrency.clone().unwrap(),
                    market.quoteCurrency.clone().unwrap()
                )
            } else {
                // linear future
                format!("{}/USD", market.underlying.clone().unwrap())
            }
        } else {
            panic!("Unknown symbol {}", market.name);
        };

        assert_eq!(pair.as_str(), pair_expected);
    }
}
